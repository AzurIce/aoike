use actix_web::{App, HttpServer};
use std::path::PathBuf;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

pub const POST_DIR: &str = "posts";
pub const SITE_DIR: &str = "site";
pub const THEMES_DIR: &str = "themes";
pub const THEME: &str = "aoike";

pub fn serve(src_dir: &PathBuf) {
    let post_dir = src_dir.join(POST_DIR);
    let site_dir = src_dir.join(SITE_DIR);
    let _theme_dir = src_dir.join(THEMES_DIR).join(THEME);

    super::build(src_dir);

    let _src_dir = src_dir.clone();
    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
           Ok(event) => {
            // 执行 build 函数
            super::build(&_src_dir);

            // 输出变化事件
            println!("{:?}", event);
        },
           Err(e) => println!("watch error: {:?}", e),
        }
    }).expect("cannot create watcher");

    watcher.watch(&post_dir, RecursiveMode::Recursive).expect("cannot watch");

    println!("[command/serve]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        HttpServer::new(move || {
            App::new().service(
                actix_files::Files::new("/", site_dir.to_str().unwrap()).show_files_listing(),
            )
        })
        .bind(("0.0.0.0", 8080))
        .expect("cannot bind")
        .run().await
    }
    ).expect("failed to block_on");
}
