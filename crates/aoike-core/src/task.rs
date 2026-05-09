use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use pulldown_cmark::{Event, Parser, TagEnd, Options};
use regex::Regex;
use tokio::sync::broadcast;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    Done,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Task {
    pub content: String,
    pub status: TaskStatus,
    pub line_number: usize,
    pub file_path: PathBuf,
    pub raw_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskUpdate {
    pub tasks: Vec<Task>,
    pub total_todo: usize,
    pub total_done: usize,
}

pub struct TaskIndex {
    pub tasks_by_file: Arc<Mutex<HashMap<PathBuf, Vec<Task>>>>,
    pub tx: broadcast::Sender<TaskUpdate>,
}

impl Clone for TaskIndex {
    fn clone(&self) -> Self {
        Self {
            tasks_by_file: Arc::clone(&self.tasks_by_file),
            tx: self.tx.clone(),
        }
    }
}

impl TaskIndex {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            tasks_by_file: Arc::new(Mutex::new(HashMap::new())),
            tx,
        }
    }

    pub fn scan_file(
        &self,
        file_path: &Path,
        content: &str,
    ) {
        let tasks = parse_tasks_from_content(file_path, content);
        
        let mut tasks_by_file = self.tasks_by_file.lock().unwrap();
        tasks_by_file.insert(file_path.to_path_buf(), tasks);
        
        drop(tasks_by_file);
        self.broadcast_update();
    }

    pub fn remove_file(
        &self,
        file_path: &Path,
    ) {
        let mut tasks_by_file = self.tasks_by_file.lock().unwrap();
        tasks_by_file.remove(file_path);
        
        drop(tasks_by_file);
        self.broadcast_update();
    }

    pub fn get_all_tasks(
        &self,
    ) -> Vec<Task> {
        let tasks_by_file = self.tasks_by_file.lock().unwrap();
        let mut all_tasks = Vec::new();
        
        for tasks in tasks_by_file.values() {
            all_tasks.extend(tasks.clone());
        }
        
        // Sort by file path and line number
        all_tasks.sort_by(|a, b| {
            a.file_path.cmp(&b.file_path)
                .then(a.line_number.cmp(&b.line_number))
        });
        
        all_tasks
    }

    pub fn get_stats(
        &self,
    ) -> (usize, usize) {
        let all_tasks = self.get_all_tasks();
        let total_todo = all_tasks.iter()
            .filter(|t| t.status == TaskStatus::Todo)
            .count();
        let total_done = all_tasks.iter()
            .filter(|t| t.status == TaskStatus::Done)
            .count();
        (total_todo, total_done)
    }

    fn broadcast_update(
        &self,
    ) {
        let all_tasks = self.get_all_tasks();
        let (total_todo, total_done) = self.get_stats();
        
        let update = TaskUpdate {
            tasks: all_tasks,
            total_todo,
            total_done,
        };
        
        let _ = self.tx.send(update);
    }

    pub fn subscribe(
        &self,
    ) -> broadcast::Receiver<TaskUpdate> {
        self.tx.subscribe()
    }
}

fn parse_tasks_from_content(
    file_path: &Path,
    content: &str,
) -> Vec<Task> {
    let mut tasks = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(content, options);
    
    let mut in_task = false;
    let mut task_checked = false;
    let mut task_start_line = 0;
    let mut task_text_parts: Vec<String> = Vec::new();
    
    // Use offset_iter to get byte offsets for each event
    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::TaskListMarker(checked) => {
                in_task = true;
                task_checked = checked;
                task_text_parts.clear();
                
                // Calculate line number from byte offset
                task_start_line = content[..range.start].chars().filter(|&c| c == '\n').count() + 1;
            }
            Event::Text(text) if in_task => {
                task_text_parts.push(text.to_string());
            }
            Event::Code(code) if in_task => {
                task_text_parts.push(format!("`{}`", code));
            }
            Event::End(TagEnd::Item) if in_task => {
                let raw_text = task_text_parts.join("");
                let (task_content, priority, due_date, tags) = parse_task_attributes(&raw_text);
                
                // Find the actual raw line for this task
                let raw_line = find_raw_task_line(content, task_start_line);
                
                tasks.push(Task {
                    content: task_content,
                    status: if task_checked { TaskStatus::Done } else { TaskStatus::Todo },
                    line_number: task_start_line,
                    file_path: file_path.to_path_buf(),
                    raw_text: raw_line,
                    priority,
                    due_date,
                    tags,
                });
                
                in_task = false;
            }
            _ => {}
        }
    }
    
    tasks
}

fn find_raw_task_line(content: &str, line_number: usize) -> String {
    content.lines()
        .nth(line_number.saturating_sub(1))
        .unwrap_or("")
        .to_string()
}

fn parse_task_attributes(
    content: &str,
) -> (String, Option<String>, Option<String>, Vec<String>) {
    let mut main_content = content.to_string();
    let mut priority = None;
    let mut due_date = None;
    let mut tags = Vec::new();
    
    // Parse priority: ⏫ 🔼 🔽
    if main_content.contains("⏫") {
        priority = Some("high".to_string());
        main_content = main_content.replace("⏫", "").trim().to_string();
    } else if main_content.contains("🔼") {
        priority = Some("medium".to_string());
        main_content = main_content.replace("🔼", "").trim().to_string();
    } else if main_content.contains("🔽") {
        priority = Some("low".to_string());
        main_content = main_content.replace("🔽", "").trim().to_string();
    }
    
    // Parse due date: 📅 2026-05-09 or due::2026-05-09
    let due_regex = Regex::new(r"(?:📅|due::)\s*(\d{4}-\d{2}-\d{2})").unwrap();
    if let Some(caps) = due_regex.captures(&main_content) {
        due_date = Some(caps[1].to_string());
        main_content = due_regex.replace(&main_content, "").trim().to_string();
    }
    
    // Parse tags: #tag
    let tag_regex = Regex::new(r"#(\w+)").unwrap();
    for caps in tag_regex.captures_iter(&main_content) {
        tags.push(caps[1].to_string());
    }
    main_content = tag_regex.replace_all(&main_content, "").trim().to_string();
    
    (main_content, priority, due_date, tags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_tasks_basic() {
        let file_path = PathBuf::from("test.md");
        let content = r#"# Notes

- [ ] Task one
- [x] Task two done
- Regular item
- [ ] Task three
"#;
        
        let tasks = parse_tasks_from_content(&file_path, content);
        assert_eq!(tasks.len(), 3, "Should find 3 tasks");
        
        assert_eq!(tasks[0].content, "Task one");
        assert_eq!(tasks[0].status, TaskStatus::Todo);
        assert_eq!(tasks[0].line_number, 3);
        
        assert_eq!(tasks[1].content, "Task two done");
        assert_eq!(tasks[1].status, TaskStatus::Done);
        assert_eq!(tasks[1].line_number, 4);
        
        assert_eq!(tasks[2].content, "Task three");
        assert_eq!(tasks[2].status, TaskStatus::Todo);
    }

    #[test]
    fn test_parse_tasks_with_attributes() {
        let file_path = PathBuf::from("test.md");
        let content = r#"- [ ] High priority ⏫ #urgent
- [x] With date 📅 2026-05-10 #work
- [ ] Multi #tag #test
"#;
        
        let tasks = parse_tasks_from_content(&file_path, content);
        assert_eq!(tasks.len(), 3);
        
        assert_eq!(tasks[0].content, "High priority");
        assert_eq!(tasks[0].priority, Some("high".to_string()));
        assert_eq!(tasks[0].tags, vec!["urgent"]);
        assert_eq!(tasks[0].status, TaskStatus::Todo);
        
        assert_eq!(tasks[1].content, "With date");
        assert_eq!(tasks[1].due_date, Some("2026-05-10".to_string()));
        assert_eq!(tasks[1].tags, vec!["work"]);
        assert_eq!(tasks[1].status, TaskStatus::Done);
        
        assert_eq!(tasks[2].content, "Multi");
        assert_eq!(tasks[2].tags, vec!["tag", "test"]);
    }

    #[test]
    fn test_parse_tasks_with_code() {
        let file_path = PathBuf::from("test.md");
        let content = "- [ ] Task with `code` inline\n";
        
        let tasks = parse_tasks_from_content(&file_path, content);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].content, "Task with `code` inline");
    }

    #[test]
    fn test_parse_tasks_empty() {
        let file_path = PathBuf::from("test.md");
        
        // No tasks
        let tasks = parse_tasks_from_content(&file_path, "# Just a heading\n\nSome text.\n");
        assert_eq!(tasks.len(), 0);
        
        // Empty task
        let tasks = parse_tasks_from_content(&file_path, "- [ ]\n");
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].content, "");
        assert_eq!(tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn test_parse_task_attributes() {
        // Priority
        let (content, priority, _, _) = parse_task_attributes("High priority task ⏫");
        assert_eq!(content, "High priority task");
        assert_eq!(priority, Some("high".to_string()));
        
        let (content, priority, _, _) = parse_task_attributes("Medium priority 🔼");
        assert_eq!(content, "Medium priority");
        assert_eq!(priority, Some("medium".to_string()));
        
        let (content, priority, _, _) = parse_task_attributes("Low priority 🔽");
        assert_eq!(content, "Low priority");
        assert_eq!(priority, Some("low".to_string()));
        
        // Due date
        let (content, _, due, _) = parse_task_attributes("Task with date 📅 2026-05-10");
        assert_eq!(content, "Task with date");
        assert_eq!(due, Some("2026-05-10".to_string()));
        
        // Tags
        let (content, _, _, tags) = parse_task_attributes("Task #work #urgent");
        assert_eq!(content, "Task");
        assert_eq!(tags, vec!["work", "urgent"]);
        
        // Combined
        let (content, priority, due, tags) = 
            parse_task_attributes("Complex task ⏫ 📅 2026-05-10 #work #aoike");
        assert_eq!(content, "Complex task");
        assert_eq!(priority, Some("high".to_string()));
        assert_eq!(due, Some("2026-05-10".to_string()));
        assert_eq!(tags, vec!["work", "aoike"]);
    }

    #[test]
    fn test_task_index() {
        let index = TaskIndex::new();
        let file_path = PathBuf::from("test.md");
        
        // Scan a file
        index.scan_file(&file_path, "- [ ] Task 1\n- [x] Task 2\n");
        
        let tasks = index.get_all_tasks();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].content, "Task 1");
        assert_eq!(tasks[0].status, TaskStatus::Todo);
        assert_eq!(tasks[1].content, "Task 2");
        assert_eq!(tasks[1].status, TaskStatus::Done);
        
        let (todo, done) = index.get_stats();
        assert_eq!(todo, 1);
        assert_eq!(done, 1);
        
        // Remove file
        index.remove_file(&file_path);
        let tasks = index.get_all_tasks();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_task_index_multiple_files() {
        let index = TaskIndex::new();
        
        index.scan_file(
            &PathBuf::from("file1.md"),
            "- [ ] Task from file 1\n"
        );
        index.scan_file(
            &PathBuf::from("file2.md"),
            "- [x] Task from file 2\n"
        );
        
        let tasks = index.get_all_tasks();
        assert_eq!(tasks.len(), 2);
        
        // Should be sorted by file path
        assert!(tasks[0].file_path.to_string_lossy().contains("file1"));
        assert!(tasks[1].file_path.to_string_lossy().contains("file2"));
    }
}