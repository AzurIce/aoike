{
  description = "aoike";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    dioxus-cli.url = "github:DioxusLabs/dioxus/v0.7.0-rc.0";
    typst.url = "github:typst/typst";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, dioxus-cli, typst, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-tools = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          # prioritize system clang, see https://github.com/zed-industries/zed/issues/7036
          # https://github.com/gfx-rs/gfx/issues/2309
          # https://mac.install.guide/commandlinetools/7
          shellHook = ''
            export PATH=/usr/bin:$PATH
          '';

          buildInputs = with pkgs; [ clang ] ++ [ rust-tools ];
          packages = [
            typst.packages.${system}.default
            dioxus-cli.packages.${system}.dioxus-cli
          ] ++ (with pkgs; [
            bun
            wasm-bindgen-cli
            libiconv
          ]);
        };
      });
}
