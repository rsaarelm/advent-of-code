let
  mozilla = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
    );
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in

with nixpkgs; mkShell {
  buildInputs = [
    just

    # Clojure
    clojure jdk11_headless clojure-lsp

    # Rust
    rustc cargo rustfmt rust-analyzer clippy
    # nixpkgs.latest.rustChannels.nightly.rust

    # Python
    python3 python3Packages.python-language-server
  ];

  RUST_SRC_PATH="${rustPlatform.rustcSrc}";
}
