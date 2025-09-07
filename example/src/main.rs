use aoike::{
    Site, app::{AoikeApp, App, HeaderContext}
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
        .with_context(HeaderContext {
            favicon: Some(FAVICON),
            main_css: Some(MAIN_CSS),
            tailwind_css: Some(TAILWIND_CSS),
        })
        .launch();
    // dioxus::launch(App);
}
