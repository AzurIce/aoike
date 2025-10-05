use dioxus::prelude::*;
use tracing::info;

use crate::app::{ConfigContext, Route};

// const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const UNO_CSS: Asset = asset!("/assets/uno.css");

#[component]
pub fn Head() -> Element {
    let header_context = consume_context::<ConfigContext>();

    rsx! {
        // style { {TAILWIND_CSS} }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: UNO_CSS }

        if let Some(href) = header_context.favicon {
            document::Link { rel: "icon", href }
        }
        if let Some(extra_head) = header_context.extra_head {
            {extra_head.as_ref()()}
        }
    }
}

// MARK: Base
#[component]
pub fn Base() -> Element {
    let mounted = use_signal(|| false);

    // INFO: Temporary solution to avoid flash of unstyled content
    {
        let mut mounted = mounted.clone();
        use_effect(move || {
            info!("use effect");
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(200).await;
                info!("set mounted to true");
                mounted.set(true);
            });
        });
    }

    rsx! {
        Head {}

        if mounted() {
            style { "body {{ opacity: 1; transition: opacity 0.6s ease; }}" }
        } else {
            style { "body {{ opacity: 0; }}" }
        }

        Header { }

        if mounted() {
            main {
                class: "max-w-[80ch] w-full m-x-auto flex flex-col items-center p-8 gap-4",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn Header() -> Element {
    let config = consume_context::<ConfigContext>();

    rsx! {
        header {
            class: "flex sticky top-0 w-full bg-transparent z-800",
            div {
                class: "absolute size-full z-[-1] border-b border-b-slate-300 bg-white/90 backdrop-blur-md"
            }
            nav {
                class: "flex gap-2 items-center p-x-6 max-w-5xl h-14 w-full m-x-auto",
                a {
                    class: "flex gap-2 m-r-auto nav-btn h-10 p-1 group",
                    href: "/",
                    {
                        config.avatar.map(|avatar| {
                            rsx! {
                                img {
                                    class: "h-full rounded",
                                    src: avatar,
                                    alt: "avatar"
                                }
                            }
                        })
                    }
                    div {
                        class: "flex flex-col",
                        span {
                            class: "text-sm transition-transform duration-500 group-hover:-translate-y-1",
                            {config.title.as_deref().unwrap_or("Site Title")}
                        }
                        span {
                            class: "text-xs text-slate-600 opacity-0 max-h-0 overflow-hidden transition-all duration-500 group-hover:opacity-100 group-hover:max-h-8",
                            {config.desc.as_deref().unwrap_or("site description")}
                        }
                    }
                }

                a {
                    class: "h-10 gap-1 nav-btn text-sm p-x-4",
                    href: "/posts",
                    "文章"
                }

                a {
                    class: "h-10 gap-1 nav-btn text-sm p-x-4",
                    href: "/search",
                    "搜索"
                }
                {
                    config.github_owner.zip(config.github_repo).map(|(owner, repo)| {
                        rsx! {
                            a {
                                class: "size-10 gap-1 nav-btn",
                                href: format!("https://github.com/{}/{}", owner, repo),
                                rel: "noreferrer",
                                div {  class: "i-fa6-brands-github text-2xl" }
                            }
                        }
                    })
                }
            }
        }
    }
}

// MARK: Blog
