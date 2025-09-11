use aoike::{
    app::{AoikeApp, App, ConfigContext},
    RsxFn, Site,
};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const AVATAR: Asset = asset!("/assets/avatar.jpg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");
    AoikeApp::default()
        .with_context(Site {
            blogs: blogs::BLOGS.clone(),
        })
        .with_context(ConfigContext {
            title: Some("冰弦のBlog".to_string()),
            desc: Some("『看清世界的真相后仍热爱生活』".to_string()),
            // author: Some("Azur冰弦".to_string()),
            email: Some("973562770@qq.com".to_string()),
            favicon: Some(FAVICON),
            avatar: Some(AVATAR),
            extra_head: Some(RsxFn::new(|| {
                rsx! {
                    document::Link { rel: "stylesheet", href: MAIN_CSS }
                    document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                }
            })),
            ..Default::default()
        })
        .launch();
    // dioxus::launch(App);
}
