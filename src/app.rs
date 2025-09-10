use std::any::Any;

use dioxus::{core::Element, prelude::*};

use crate::{BlogData, Site};

#[derive(Default, Clone)]
pub struct HeaderContext {
    pub favicon: Option<Asset>,
    pub main_css: Option<Asset>,
    pub tailwind_css: Option<Asset>,
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
    Home, // <---- a DogView component must be in scope
    #[route("/blog/:slug")]
    Blog { slug: String },
}

#[component]
pub fn Blog(slug: String) -> Element {
    let header_context = consume_context::<HeaderContext>();
    let blogs = consume_context::<Site>().blogs;

    rsx! {
        if let Some(href) = header_context.favicon {
            document::Link { rel: "icon", href }
        }
        if let Some(href) = header_context.main_css {
            document::Link { rel: "stylesheet", href }
        }
        if let Some(href) = header_context.tailwind_css {
            document::Link { rel: "stylesheet", href }
        }

        div {
            class: "blogs-container",
            for blog in blogs {
                BlogCard { blog }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    let header_context = consume_context::<HeaderContext>();
    let blogs = consume_context::<Site>().blogs;

    rsx! {
        if let Some(href) = header_context.favicon {
            document::Link { rel: "icon", href }
        }
        if let Some(href) = header_context.main_css {
            document::Link { rel: "stylesheet", href }
        }
        if let Some(href) = header_context.tailwind_css {
            document::Link { rel: "stylesheet", href }
        }

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
            class: "blog-card",
            "{blog.title}"
            {blog.summary_rsx.as_ref()()}
        }
    }
}
