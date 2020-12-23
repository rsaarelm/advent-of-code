let
  mozilla = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
    );
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in

with nixpkgs; mkShell {
  buildInputs = [
    babashka just

    # Clojure
    clojure jdk11_headless clojure-lsp

    # Rust
    nixpkgs.latest.rustChannels.nightly.rust
    cargo rustfmt rust-analyzer clippy

    # Python
    python3
  ];

  RUST_SRC_PATH="${rustPlatform.rustcSrc}";
}
