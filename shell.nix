let
  rust-overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
    # builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz
    );
  nixpkgs = import <nixpkgs> { overlays = [ rust-overlay ]; };
in

with nixpkgs; mkShell {
  buildInputs = [
    just

    # Clojure
    clojure jdk11_headless clojure-lsp

    # Rust
    # rustc cargo rustfmt clippy rust-analyzer
    nixpkgs.latest.rustChannels.nightly.rust
    nixpkgs.latest.rustChannels.nightly.rust-src

    # Python
    python3
    python3Packages.numpy
    python3Packages.python-lsp-server
    python3Packages.pylsp-mypy
    black
  ];

  # For stable rustc
  # RUST_SRC_PATH="${rustPlatform.rustcSrc}";

  # For nightly rustc from Mozilla overlay
  RUST_SRC_PATH="${nixpkgs.latest.rustChannels.nightly.rust-src}/lib/rustlib/src/rust/library";
  RUST_BACKTRACE=1;
}
