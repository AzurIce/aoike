# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Aoike is an experimental static site generator built on Rust and `build.rs`. The core philosophy is "the site can be abstracted into pure data structures" - content is processed at build time into Rust data structures, then rendered as a static site using WASM-based UI frameworks.

This is a workspace with multiple members:
- **Root package (`aoike`)**: Core library providing data structures and build utilities
- **`packages/aoike-dioxus`**: Dioxus framework integration and `AoikeApp` implementation
- **`packages/aoike-sycamore`**: Sycamore framework integration and `AoikeApp` implementation (recommended)
- **`example/dioxus`**: Basic Dioxus example
- **`example/dioxus-docsgen`**: Example demonstrating markdown-to-blog pipeline with Dioxus
- **`example/sycamore`**: Example using Sycamore framework

## Architecture

### Core Concepts

1. **Framework-agnostic core** (`aoike`):
   - `Site`: Contains static references to all posts and index content
   - `PostData`: Individual post with title, slug, HTML content (both summary and full), created/updated timestamps
   - `build` feature: Provides build-time utilities for parsing and code generation

2. **Framework integrations**:
   - **`aoike-dioxus`**: Dioxus-specific implementation
     - `RsxFn`: Wrapper around `Arc<dyn Fn() -> Element>` for storing pre-compiled Dioxus RSX
     - `PostData`: Extended with `summary_rsx` and `content_rsx` fields
     - `AoikeApp`: Built-in app with routing, blog layout, Giscus comments integration
     - `build` feature: Includes `dioxus-rsx-rosetta` for HTML → RSX conversion

   - **`aoike-sycamore`**: Sycamore-specific implementation (recommended)
     - Similar structure to Dioxus version, optimized for Sycamore framework
     - Uses View instead of Element for rendering

3. **Build-time generation workflow** (see `example/dioxus-docsgen/build.rs`):
   - Read markdown files from `doc-src/`
   - Parse markdown to HTML using `pulldown-cmark`
   - For Dioxus: Convert HTML to RSX using `dioxus-rsx-rosetta`
   - Extract git timestamps (created/updated) for each file
   - Generate code file with static data structures
   - Summary extraction: removes H1 tags and limits to first 200 characters

### Styling

- **SCSS**: Compiled at build time using `rsass`
  - Root package: `build.rs` may compile shared styles
  - Framework packages: Each has its own `build.rs` for framework-specific styles
- **CSS output**: Now exported to `static/css/` directory (previously `assets/css/`)
- **Asset injection**: Framework packages support automatic CSS injection and asset copying

## Development Commands

### Example development
```bash
# Dioxus examples
cd example/dioxus/
dx serve  # Dioxus dev server with hot reload

cd example/dioxus-docsgen/
dx serve

# Sycamore example
cd example/sycamore/
trunk serve  # or equivalent Sycamore dev server
```

### Building for production
Each framework handles builds differently:
- **Dioxus**: `dx build` (uses Dioxus CLI)
- **Sycamore**: `trunk build` (uses Trunk)

The `build.rs` files automatically run during the build process to:
- Compile SCSS to CSS
- Parse markdown and generate Rust code

## Key Files

### Core library (`aoike`)
- `src/lib.rs`: Core data structures (`Site`, `PostData`)
- `src/build.rs`: Build-time utilities for parsing and codegen

### Dioxus integration (`packages/aoike-dioxus`)
- `src/lib.rs`: Dioxus-specific data structures (`RsxFn`, extended `PostData`)
- `src/app.rs`: `AoikeApp` implementation with routing and components
- `src/components/giscus.rs`: Giscus comments integration
- `src/build.rs`: Dioxus-specific build utilities (HTML → RSX conversion)
- `build.rs`: SCSS compilation for Dioxus

### Sycamore integration (`packages/aoike-sycamore`)
- Similar structure to Dioxus package
- `build.rs`: Handles CSS bundling using zip archive approach

## Notes

- **Framework choice**: `aoike-sycamore` is recommended over `aoike-dioxus`
- **Static generation**: All post data is embedded in the WASM binary
- **Git timestamps**: Git is used at build time to extract file timestamps - ensure files are committed for accurate dates
- **Feature flags**:
  - `build` feature enables build-time dependencies (`quote`, `proc-macro2`, etc.)
  - Optional features control framework-specific build tools
- **CSS bundling**: Recent change moves CSS from `assets/css` to `static/css`
- **Index injection**: Framework packages support injecting content into `index.html`
