let statsChart = null;
let currentTasks = [];
let currentTaskFilter = 'all';
let currentTagFilter = null;

// Stats functions
async function fetchStats() {
    try {
        const response = await fetch('/api/stats');
        if (!response.ok) throw new Error('Failed to fetch stats');
        const data = await response.json();
        updateStatsUI(data);
    } catch (error) {
        console.error('Error fetching stats:', error);
    }
}

function updateStatsUI(data) {
    const entries = Object.entries(data.counts)
        .sort((a, b) => b[1] - a[1]);
    
    document.getElementById('total-files').textContent = data.total;
    document.getElementById('total-types').textContent = entries.length;
    document.getElementById('top-type').textContent = entries.length 
        ? `.${entries[0][0]}` 
        : '-';
    
    const tbody = document.getElementById('stats-tbody');
    tbody.innerHTML = '';
    
    entries.forEach(([ext, count]) => {
        const percentage = data.total > 0 
            ? ((count / data.total) * 100).toFixed(1) + '%'
            : '0%';
        
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><span class="badge">.${ext}</span></td>
            <td class="count">${count}</td>
            <td class="percentage">${percentage}</td>
        `;
        tbody.appendChild(row);
    });
    
    updateChart(entries);
}

function updateChart(entries) {
    const ctx = document.getElementById('stats-chart').getContext('2d');
    
    const labels = entries.map(([ext]) => `.${ext}`);
    const data = entries.map(([_, count]) => count);
    
    const colors = [
        '#18181b', '#3f3f46', '#71717a', '#a1a1aa',
        '#d4d4d8', '#e4e4e7', '#f4f4f5', '#fafafa'
    ];
    
    if (statsChart) {
        statsChart.destroy();
    }
    
    statsChart = new Chart(ctx, {
        type: 'doughnut',
        data: {
            labels: labels,
            datasets: [{
                data: data,
                backgroundColor: colors.slice(0, entries.length),
                borderWidth: 2,
                borderColor: '#ffffff',
                hoverOffset: 4
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            cutout: '65%',
            plugins: {
                legend: {
                    display: false
                },
                tooltip: {
                    backgroundColor: '#18181b',
                    padding: 12,
                    cornerRadius: 8,
                    titleFont: {
                        size: 13,
                        family: '-apple-system, BlinkMacSystemFont, sans-serif'
                    },
                    bodyFont: {
                        size: 13,
                        family: '-apple-system, BlinkMacSystemFont, sans-serif'
                    },
                    callbacks: {
                        label: (context) => {
                            const total = context.dataset.data.reduce((a, b) => a + b, 0);
                            const percentage = ((context.raw / total) * 100).toFixed(1);
                            return ` ${context.raw} files (${percentage}%)`;
                        }
                    }
                }
            },
            animation: {
                duration: 400
            }
        }
    });
}

// Task functions
async function fetchTasks() {
    try {
        const response = await fetch('/api/tasks');
        if (!response.ok) throw new Error('Failed to fetch tasks');
        const data = await response.json();
        console.log('Fetched tasks:', data);
        currentTasks = data.tasks || [];
        updateTasksUI(data);
    } catch (error) {
        console.error('Error fetching tasks:', error);
    }
}

function updateTasksUI(data) {
    const total = (data.total_todo || 0) + (data.total_done || 0);
    const completionRate = total > 0 
        ? ((data.total_done / total) * 100).toFixed(0) + '%'
        : '0%';
    
    document.getElementById('total-tasks').textContent = total;
    document.getElementById('todo-count').textContent = data.total_todo;
    document.getElementById('done-count').textContent = data.total_done;
    document.getElementById('completion-rate').textContent = completionRate;
    
    renderTagCloud();
    renderTaskList();
}

function getAllTags() {
    const tagCounts = new Map();
    currentTasks.forEach(task => {
        if (task.tags && Array.isArray(task.tags)) {
            task.tags.forEach(tag => {
                tagCounts.set(tag, (tagCounts.get(tag) || 0) + 1);
            });
        }
    });
    return Array.from(tagCounts.entries())
        .sort((a, b) => b[1] - a[1]);
}

function renderTagCloud() {
    const container = document.getElementById('tag-cloud');
    if (!container) return;
    
    container.innerHTML = '';
    
    const tags = getAllTags();
    if (tags.length === 0) return;
    
    // Add "All Tags" button
    const allBtn = document.createElement('div');
    allBtn.className = `tag-cloud-item ${currentTagFilter === null ? 'active' : ''}`;
    allBtn.textContent = 'All Tags';
    allBtn.addEventListener('click', () => {
        currentTagFilter = null;
        renderTagCloud();
        renderTaskList();
    });
    container.appendChild(allBtn);
    
    tags.forEach(([tag, count]) => {
        const item = document.createElement('div');
        item.className = `tag-cloud-item ${currentTagFilter === tag ? 'active' : ''}`;
        item.innerHTML = `#${tag}<span class="count">${count}</span>`;
        item.addEventListener('click', () => {
            currentTagFilter = currentTagFilter === tag ? null : tag;
            renderTagCloud();
            renderTaskList();
        });
        container.appendChild(item);
    });
}

function renderTaskList() {
    const container = document.getElementById('task-list');
    if (!container) {
        console.error('task-list container not found');
        return;
    }
    
    container.innerHTML = '';
    
    console.log('Rendering tasks:', currentTasks);
    
    if (!Array.isArray(currentTasks)) {
        console.error('currentTasks is not an array:', currentTasks);
        container.innerHTML = '<div style="text-align: center; color: var(--text-muted); padding: 2rem;">Error loading tasks</div>';
        return;
    }
    
    const filteredTasks = currentTasks.filter(task => {
        if (!task || !task.status) return false;
        
        // Status filter
        if (currentTaskFilter === 'todo' && task.status !== 'todo') return false;
        if (currentTaskFilter === 'done' && task.status !== 'done') return false;
        
        // Tag filter
        if (currentTagFilter !== null) {
            const taskTags = task.tags || [];
            if (!taskTags.includes(currentTagFilter)) return false;
        }
        
        return true;
    });
    
    if (filteredTasks.length === 0) {
        container.innerHTML = '<div style="text-align: center; color: var(--text-muted); padding: 2rem;">No tasks found</div>';
        return;
    }
    
    filteredTasks.forEach(task => {
        const item = document.createElement('div');
        item.className = `task-item ${task.status}`;
        
        const checkbox = task.status === 'done' ? '✓' : '';
        const priorityClass = task.priority ? `task-priority-${task.priority}` : '';
        
        const tagsHtml = (task.tags || []).map(tag => 
            `<span class="task-tag">#${tag}</span>`
        ).join('');
        
        const dueHtml = task.due_date 
            ? `<span class="task-tag" style="color: var(--warning);">📅 ${task.due_date}</span>`
            : '';
        
        const fileName = task.file_path.split(/[\\/]/).pop();
        
        item.innerHTML = `
            <div class="task-checkbox">${checkbox}</div>
            <div class="task-content">
                <div class="task-text ${priorityClass}">${escapeHtml(task.content)}</div>
                <div class="task-meta">
                    ${tagsHtml}
                    ${dueHtml}
                    <span class="task-file">${fileName}:${task.line_number}</span>
                </div>
            </div>
        `;
        
        container.appendChild(item);
    });
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Tab switching
function setupTabs() {
    document.querySelectorAll('.tab').forEach(tab => {
        tab.addEventListener('click', () => {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
            
            tab.classList.add('active');
            document.getElementById(`${tab.dataset.tab}-tab`).classList.add('active');
        });
    });
}

// Task filter buttons
function setupFilters() {
    document.querySelectorAll('.filter-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            currentTaskFilter = btn.dataset.filter;
            renderTaskList();
        });
    });
}

// SSE setup
function setupSSE() {
    // Stats SSE
    const statsEventSource = new EventSource('/api/events');
    
    statsEventSource.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            updateStatsUI(data);
        } catch (error) {
            console.error('Error parsing stats SSE data:', error);
        }
    };
    
    statsEventSource.onerror = () => {
        document.getElementById('status-indicator').style.background = '#ef4444';
        setTimeout(() => {
            statsEventSource.close();
            setupSSE();
        }, 3000);
    };
    
    statsEventSource.onopen = () => {
        document.getElementById('status-indicator').style.background = '#22c55e';
    };
    
    // Tasks SSE
    const tasksEventSource = new EventSource('/api/task-events');
    
    tasksEventSource.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            console.log('SSE tasks update:', data);
            currentTasks = data.tasks || [];
            updateTasksUI(data);
        } catch (error) {
            console.error('Error parsing tasks SSE data:', error);
        }
    };
    
    tasksEventSource.onerror = () => {
        setTimeout(() => {
            tasksEventSource.close();
        }, 3000);
    };
}

// Initialize on DOM ready
document.addEventListener('DOMContentLoaded', () => {
    console.log('App initialized');
    setupTabs();
    setupFilters();
    fetchStats();
    fetchTasks();
    setupSSE();
});

// Handle window resize
window.addEventListener('resize', () => {
    if (statsChart) {
        statsChart.resize();
    }
});
