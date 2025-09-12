pub mod layout;

use std::any::Any;

use dioxus::{core::Element, prelude::*};

use crate::{
    app::layout::Base,
    components::giscus::{Giscus, GiscusOptions, InputPosition},
    PostData, RsxFn, Site,
};

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

#[derive(Default, Clone)]
pub struct ConfigContext {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub email: Option<String>,
    pub favicon: Option<Asset>,
    pub avatar: Option<Asset>,
    pub github_owner: Option<String>,
    pub github_repo: Option<String>,
    pub bilibili_url: Option<String>,
    pub steam_url: Option<String>,
    pub extra_head: Option<RsxFn>,
    pub giscus_options: Option<GiscusOptions>,
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
    #[layout(Base)]
    #[route("/")]
    Home,
    #[route("/posts")]
    Posts,
    #[route("/posts/:slug")]
    Post { slug: String },
    #[route("/404")]
    NotFound,
}

#[component]
pub fn Posts() -> Element {
    let posts = consume_context::<Site>().posts;

    rsx! {
        for post in posts {
            BlogCard { post }
        }
    }
}

#[component]
pub fn Post(slug: String) -> Element {
    let posts = consume_context::<Site>().posts;

    let post = posts.iter().find(|b| b.slug == slug);
    let Some(post) = post else {
        navigator().replace(Route::NotFound);
        return rsx! {};
    };

    rsx! {
        div {
            class: "markdown",
            {post.content_rsx.as_ref()()}
        }
    }
}

// MARK: Notfound
#[component]
pub fn NotFound() -> Element {
    rsx! {
        h1 { "404 Not Found" }
    }
}

#[component]
pub fn Hero() -> Element {
    let config = consume_context::<ConfigContext>();

    rsx! {
        div {
            class: "flex items-stretch",
            {config.avatar.map(|a| {
                rsx! {
                    img {class: "size-40 rounded", src: "{a}"}
                }
            })}
            div {
                class: "flex flex-col items-center justify-around p-2 p-b-1 gap-3",
                // 标题
                {let title = config.title.as_deref().unwrap_or("Site Title");
                rsx! {
                    span {
                        class: "text-xl lxgw",
                        "< {title} />"
                    }
                }}
                // 描述
                {let desc = config.desc.as_deref().unwrap_or("site description");
                rsx! {
                    span {
                        class: "text-sm lxgw",
                        "{desc}"
                    }
                }}
                // 邮箱
                {config.email.map(|mail| {
                    rsx! {
                        span {
                            class: "text-sm",
                            "📫 "
                            a {
                                class: "underline",
                                href: "mailto:{mail}",
                                "{mail}"
                            }
                        }
                    }
                })}
                // 社交媒体链接
                div {
                    class: "flex",
                    // GitHub
                    {config.github_owner.map(|owner| {
                        rsx! {
                            a {
                                href: "https://github.com/{owner}",
                                target: "_blank",
                                rel: "noreferrer",
                                class: "size-8 gap-1 nav-btn",
                                div {
                                    class: "i-fa6-brands-github text-xl"
                                }
                            }
                        }
                    })}
                    // Bilibili
                    {config.bilibili_url.map(|url| {
                        rsx! {
                            a {
                                href: "{url}",
                                target: "_blank",
                                rel: "noreferrer",
                                class: "size-8 gap-1 nav-btn",
                                div {
                                    class: "i-fa6-brands-bilibili text-xl color-[#19a2d4] translate-x-0 translate-y-[1px]"
                                }
                            }
                        }
                    })}
                    // Steam
                    {config.steam_url.map(|url| {
                        rsx! {
                            a {
                                href: "{url}",
                                target: "_blank",
                                rel: "noreferrer",
                                class: "size-8 gap-1 nav-btn",
                                div {
                                    class: "i-fa6-brands-steam text-xl bg-[#082256]"
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    let site = consume_context::<Site>();
    let config = consume_context::<ConfigContext>();

    rsx! {
        Hero {}

        div {
            class: "flex flex-col w-full p-2 markdown",
            h2 { "最新文章" }
            ul {
                {let latest_blogs = site.posts.iter().take(5).collect::<Vec<_>>();
                rsx! {
                    for blog in latest_blogs {
                        li {
                            class: "flex gap-8",
                            span {
                                class: "text-gray-600",
                                "{blog.created.year()}-{u8::from(blog.created.month())}-{blog.created.day()}"
                            }
                            a {
                                class: "underline hover:underline-gray-400",
                                href: format!("/posts/{}", blog.slug),
                                "{blog.title}"
                            }
                        }
                    }
                }}
            }
            hr {  }
            {site.index.content_rsx.as_ref()()}
        }

        {config.giscus_options.map(|options| {
            rsx! {
                Giscus { options }
            }
        })}
    }
}

#[component]
pub fn BlogCard(post: &'static PostData) -> Element {
    rsx! {
        div {
            class: "flex flex-col p-2 rounded border border-slate-200 hover:border-slate-400",
            onclick: move |_| {
                navigator().push(format!("blog/{}", post.slug));
            },
            h1 { "{post.title}" },
            div {
                class: "flex gap-2",
                span {
                    "Created: {post.created.year()}-{u8::from(post.created.month())}-{post.created.day()}"
                }
                span {
                    "Updated: {post.updated.year()}-{u8::from(post.updated.month())}-{post.updated.day()}"
                }
            }
            div { class: "summary", {post.summary_rsx.as_ref()()} }
        }
    }
}
