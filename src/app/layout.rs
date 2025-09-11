use dioxus::prelude::*;

use crate::app::{ConfigContext, Route};

// const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const UNO_CSS: Asset = asset!("/assets/uno.css");

#[component]
pub fn Head() -> Element {
    let header_context = consume_context::<ConfigContext>();

    rsx! {
        // style { {TAILWIND_CSS} }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: UNO_CSS }

        if let Some(href) = header_context.favicon {
            document::Link { rel: "icon", href }
        }
        if let Some(extra_head) = header_context.extra_head {
            {extra_head.as_ref()()}
        }
    }
}

#[component]
pub fn Base() -> Element {
    rsx! {
        Head {}

        div {
            class: "max-w-[80ch] w-full m-x-auto flex flex-col items-center p-8 gap-4",
            Outlet::<Route> {}
        }
    }
}
