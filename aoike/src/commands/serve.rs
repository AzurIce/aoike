use axum::Router;
use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Result, Watcher};
use notify_debouncer_mini::new_debouncer;
use std::{path::PathBuf, thread::sleep, time::Duration};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

pub const POST_DIR: &str = "posts";
pub const SITE_DIR: &str = "site";
pub const THEMES_DIR: &str = "themes";
pub const THEME: &str = "aoike";

/// Start the serve server.
///
/// `src_dir`: the root of the site
pub fn serve(src_dir: &PathBuf) {
    let post_dir = src_dir.join(POST_DIR);
    let site_dir = src_dir.join(SITE_DIR);
    let theme_dir = src_dir.join(THEMES_DIR).join(THEME);

    super::build(src_dir);

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(&site_dir))
        .layer(livereload);

    // 监听，有变化重新 build
    // watcher for building
    let _src_dir = src_dir.clone();
    let mut debouncer = new_debouncer(Duration::from_secs(2), move |res| {
        match res {
            Ok(events) => {
                // 输出变化事件
                println!("{:?}", events);

                // 执行 build 函数
                super::build(&_src_dir);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    })
    .unwrap();

    debouncer
        .watcher()
        .watch(&post_dir, RecursiveMode::Recursive)
        .expect("cannot watch posts");
    debouncer
        .watcher()
        .watch(&theme_dir, RecursiveMode::Recursive)
        .expect("cannot watch themes");

    // watcher for livereload
    let mut debouncer = new_debouncer(Duration::from_secs(1), move |res| {
        match res {
            Ok(events) => {
                // 输出变化事件
                reloader.reload();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    })
    .unwrap();
    debouncer.watcher()
        .watch(&site_dir, RecursiveMode::Recursive)
        .expect("cannot watch site");

    // 启动 HttpServer
    println!("[command/serve]");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}
