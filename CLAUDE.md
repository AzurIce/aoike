# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Aoike is an experimental static site generator built on Rust, Dioxus (WASM-based UI framework), and `build.rs`. The core philosophy is "the site can be abstracted into pure data structures" - content is processed at build time into Rust data structures, then rendered as a static site.

This is a workspace with multiple members:
- **Root package (`aoike`)**: The library that provides the core framework
- **`example-docs/site`**: Example demonstrating how to build a blog from markdown files
- **`example`**: Another example implementation

## Architecture

### Core Concepts

1. **`App` trait**: Defines how an application is built and launched. Implementations provide their own routing and UI logic.

2. **`AoikeApp`**: Built-in `App` implementation that creates a blog-style site with:
   - Home page with hero section and latest posts
   - Posts listing page
   - Individual post pages
   - 404 page

3. **Data structures**:
   - `Site`: Contains static references to all posts and index content
   - `PostData`: Individual post with title, slug, RSX content (both summary and full), created/updated timestamps
   - `RsxFn`: Wrapper around `Arc<dyn Fn() -> Element>` for storing pre-compiled Dioxus RSX
   - `ConfigContext`: Site configuration (title, description, social links, Giscus comments, etc.)

4. **Build-time generation**: The `build.rs` in `example-docs/site` demonstrates the typical workflow:
   - Read markdown files from `doc-src/`
   - Parse markdown to HTML using `pulldown-cmark`
   - Convert HTML to Dioxus RSX using `dioxus-rsx-rosetta`
   - Extract git timestamps (created/updated) for each file
   - Generate `docsgen.rs` with static data structures
   - Summary extraction: removes H1 tags and limits to first 200 characters

### Styling

- **SCSS**: Compiled at build time in the root `build.rs` using `rsass` (compiles `assets/main.scss` → `assets/main.css`)
- **UnoCSS**: CSS utility framework, run via `bun dev` (watches) or `bun build`
- Transitioning from Tailwind to UnoCSS (note: `tailwind.config.js` is deleted in current git status)

## Development Commands

### Root package development
```bash
# Watch and compile UnoCSS (required for styling changes)
bun dev

# Build UnoCSS once
bun build
```

### Example site development
```bash
cd example/
dx serve  # Dioxus dev server with hot reload
```

For `example-docs/site`:
```bash
cd example-docs/site/
dx serve
```

### Building for production
Dioxus handles the build process. The `build.rs` files automatically:
- Compile SCSS to CSS (root package)
- Parse markdown and generate Rust code (example-docs)

## Key Files

- `src/lib.rs`: Core data structures (`Site`, `PostData`, `RsxFn`)
- `src/app.rs`: `App` trait and `AoikeApp` implementation with routing and components
- `src/app/layout.rs`: Base layout component
- `src/components/giscus.rs`: Giscus comments integration
- `build.rs` (root): SCSS compilation
- `example-docs/site/build.rs`: Markdown → Rust code generation pipeline

## Notes

- Context data (`Site`, `ConfigContext`) is consumed via Dioxus context API in components
- Static generation means all post data is embedded in the WASM binary
- Git is used at build time to extract file timestamps - ensure files are committed for accurate dates
- The `to_token` feature enables `quote` and `proc-macro2` for code generation in build scripts
