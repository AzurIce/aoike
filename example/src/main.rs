use aoike::{
    app::{AoikeApp, App, ConfigContext},
    RsxFn, Site,
};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");
    AoikeApp::default()
        .with_context(Site {
            blogs: blogs::BLOGS.clone(),
        })
        .with_context(ConfigContext {
            favicon: Some(FAVICON),
            extra_head: Some(RsxFn::new(|| {
                rsx! {
                    document::Link { rel: "stylesheet", href: MAIN_CSS }
                    document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                }
            })),
        })
        .launch();
    // dioxus::launch(App);
}
