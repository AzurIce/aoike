# Aoike

An experimental static site generator based on `build.rs` and [Dioxus](https://dioxuslabs.com/).

## Usage

Aoike is highly customizable, with simple configuration.

The simplest way to use it is create a package, adding `aoike` to your dependencies and use `aoike::AoikeApp` to launch an app based on Dioxus, just like `example/`:

```rust
use aoike::{
    app::{AoikeApp, App, ConfigContext},
    Site
};

fn main() {
    // Site and ConfigContext are required for aoike to build the ui and pages
    AoikeApp::default()
        .with_context(Site {
            // data...
        })
        .with_context(ConfigContext {
            // config...
        })
        .launch();
}
```

`AoikeApp` is a built-in implementation of `App`, it can convert `Site` data into a blog site. A common way to construct `Site` data is using `build.rs` to generate a rust file containing the data defination. This is shown in `examples-docs`. In the `build.rs`, we:

- Read index content from `doc-src/index.md` file.
- Read blog files from `doc-src/posts` directory.
- Use git cli to get create and update time of each file.
- Parse markdown to html with `pulldown-cmark`, then convert html to rsx.
- Assemble the `Post` data, and generate a `docsgen.rs` file containing the data defination of `Site`'s fields.

Aoike is designed with the philosophy of "the site can be abstracted into pure data structures", and the app is fully customizable through the `App` trait. So you have full control of:
- What kind of context data your app require.
- How your app is built from the context data.
- How the context data is constructed.

Usually, the common approach of constructing the context data is to use the `build.rs` file, so there are infinite possibilities:
- You can use `pulldown-cmark` to parse markdown files to html, and convert html to rsx. (just like the example)
- You can use `typst` to compile typst files to html, and convert html to rsx.
- You can write a scraper to retrive data from internet and build rsx for a statistic app.
- You can access your filesystem of assets (like videos and images), and generate a static site to show them.
- You can encrypt your html source, and implement decrypt method for it with a password input in the app.
- ...

## Development

We use unocss and sass to style the built-in app. The sass process is simply done in `build.rs`, but you'll need `bun` to install and run the unocss process.

```bash
# run in project root
bun dev
```

And you'll need a demo app to check your implementation of `App`, for example, `example-docs` and `example` for `AoikeApp`:

```bash
# run in example/
dx serve
```