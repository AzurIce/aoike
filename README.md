# Aoike

An experimental static site generator based on `build.rs`.

## Usage

Aoike is highly customizable, with simple configuration. Check the examples in `example/` for more details.

`aoike` crate provides the core data structures and the basic parsing and codegen logic.

`aoike-dioxus` and `aoike-sycamore` are the implementations of `AoikeApp` for Dioxus and Sycamore respectively.It is recommended to use `aoike-sycamore` instead of `aoike-dioxus`.

## Design Philosophy

The whole philosophy is "the site can be abstracted into pure data structures", so you can use any framework you want to build your site.

And the whole process can be divided into two phases:

1. The build phase:
    This normally happends in the `build.rs`, we:

    - Read index content from `doc-src/index.md` file.
    - Read blog files from `doc-src/posts` directory.
    - Use git cli to get create and update time of each file.
    - Parse markdown to html with `pulldown-cmark`.
    - Assemble the `Post` data, and generate a `docsgen.rs` file containing the data defination.

2. The runtime phase:
    This depends on the framework you use, for example, in Dioxus, you can use `aoike-dioxus::AoikeApp` to launch an app, and in Sycamore, you can use `aoike-sycamore::AoikeApp` to launch an app.

With this two phases, there are infinite possibilities:
- You can use `pulldown-cmark` to parse markdown files to html, and convert html to rsx. (just like the example)
- You can use `typst` to compile typst files to html, and convert html to rsx.
- You can write a scraper to retrive data from internet and build rsx for a statistic app.
- You can access your filesystem of assets (like videos and images), and generate a static site to show them.
- You can encrypt your html source, and implement decrypt method for it with a password input in the app.
- ...
