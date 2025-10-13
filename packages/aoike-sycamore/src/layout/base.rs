use sycamore::prelude::*;

use crate::ConfigContext;

#[component]
pub fn Header() -> View {
    let config = use_context::<ConfigContext>();

    let title = config.title.clone().unwrap_or("Site Title".to_string());
    let desc = config
        .desc
        .clone()
        .unwrap_or("site description".to_string());
    view! {
        header(class="flex sticky top-0 w-full bg-transparent z-800") {
            div(class="absolute size-full z-[-1] border-b border-b-slate-300 bg-white/90 backdrop-blur-md")
            nav(class="flex gap-2 items-center p-x-6 max-w-5xl h-14 w-full m-x-auto") {
                a(class="flex gap-2 m-r-auto nav-btn h-10 p-1 group", href="/") {
                    (config.avatar.clone().map(|avatar| {
                        view! {
                            img(class="h-full rounded", src=avatar, alt="avatar")
                        }
                    }))
                    div(class="flex flex-col") {
                        span(class="text-sm transition-transform duration-500 group-hover:-translate-y-1") {
                            (title)
                        }
                        span(class="text-xs text-slate-600 opacity-0 max-h-0 overflow-hidden transition-all duration-500 group-hover:opacity-100 group-hover:max-h-8") {
                            (desc)
                        }
                    }
                }
                a(class="h-10 gap-1 nav-btn text-sm p-x-4", href="/posts") {
                    "文章"
                }
                a(class="h-10 gap-1 nav-btn text-sm p-x-4", href="/search") {
                    "搜索"
                }
                (config.github_owner.clone().zip(config.github_repo.clone()).map(|(owner, repo)| {
                    view! {
                        a(class="size-10 gap-1 nav-btn", href=format!("https://github.com/{}/{}", owner, repo), rel="noreferrer") {
                            div(class="i-fa6-brands-github text-2xl")
                        }
                    }
                }))
            }
        }
    }
}
