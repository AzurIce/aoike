use std::fs;
use std::path::Path;
use pulldown_cmark::{html, Options, Parser};
use sycamore::prelude::*;

fn get_rendered_markdown<P: AsRef<Path>>(path: P) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let content = fs::read_to_string(path).expect(&*format!("Failed to read file"));
    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

#[component]
fn App<G: Html>(cx: Scope, content: String) -> View<G> {
    // let rendered_content = get_rendered_markdown("posts/old test.md");
    view! { cx,
        p(class="text-2xl") { (content) }
        // article {

        // }
    }
}

fn main() {
    let rendered_content = get_rendered_markdown("posts/old test.md");
    // println!("{}", rendered_content)
    sycamore::render(|cx| App(cx, rendered_content));
}
