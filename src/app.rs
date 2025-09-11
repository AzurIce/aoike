use std::any::Any;

use dioxus::{core::Element, prelude::*};

use crate::{BlogData, RsxFn, Site};

#[derive(Default, Clone)]
pub struct ConfigContext {
    pub favicon: Option<Asset>,
    pub extra_head: Option<RsxFn>,
}

pub trait App {
    fn builder(self) -> LaunchBuilder;
    fn app() -> Element;

    fn launch(self)
    where
        Self: Sized,
    {
        self.builder().launch(Self::app);
    }
}

// MARK: AoikeApp
pub struct AoikeApp {
    launch_builder: LaunchBuilder,
}

impl Default for AoikeApp {
    fn default() -> Self {
        AoikeApp {
            launch_builder: LaunchBuilder::new(),
        }
    }
}

impl AoikeApp {
    pub fn with_context(mut self, state: impl Any + Clone + Send + Sync + 'static) -> Self {
        self.launch_builder = self.launch_builder.with_context(state);
        self
    }
    pub fn with_context_provider(
        mut self,
        state: impl Fn() -> Box<dyn Any> + Send + Sync + 'static,
    ) -> Self {
        self.launch_builder = self.launch_builder.with_context_provider(state);
        self
    }
}

/// The main app for aoike
///
/// Required Context:
/// - [`Site`]
impl App for AoikeApp {
    fn builder(self) -> LaunchBuilder {
        self.launch_builder
    }
    fn app() -> Element {
        rsx! {
            Router::<Route> {}
        }
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/blog/:slug")]
    Blog { slug: String },
    #[route("/404")]
    NotFound,
}

// const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Head() -> Element {
    let header_context = consume_context::<ConfigContext>();

    rsx! {
        // style { {TAILWIND_CSS} }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        if let Some(href) = header_context.favicon {
            document::Link { rel: "icon", href }
        }
        if let Some(extra_head) = header_context.extra_head {
            {extra_head.as_ref()()}
        }
    }
}

#[component]
pub fn Blog(slug: String) -> Element {
    let blogs = consume_context::<Site>().blogs;

    let blog = blogs.iter().find(|b| b.slug == slug);
    let Some(blog) = blog else {
        navigator().replace(Route::NotFound);
        return rsx! {};
    };

    rsx! {
        Head {}

        div {
            class: "content",
            {blog.content_rsx.as_ref()()}
        }
    }
}

#[component]
pub fn NotFound() -> Element {
    rsx! {
        h1 { "404 Not Found" }
    }
}

#[component]
pub fn Home() -> Element {
    let blogs = consume_context::<Site>().blogs;

    rsx! {
        Head {}

        div {
            class: "blogs-container",
            for blog in blogs {
                BlogCard { blog }
            }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            // img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

#[component]
pub fn BlogCard(blog: BlogData) -> Element {
    rsx! {
        div {
            class: "blog-card p-2",
            onclick: move |_| {
                navigator().push(format!("blog/{}", blog.slug));
            },
            h1 { "{blog.title}" },
            div { class: "summary", {blog.summary_rsx.as_ref()()} }
        }
    }
}
