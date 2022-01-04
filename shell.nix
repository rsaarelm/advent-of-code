let
  rust-overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ rust-overlay ]; };
in

with nixpkgs; mkShell {
  buildInputs = [
    just

    # Clojure
    clojure jdk11_headless clojure-lsp

    # Rust
    # rustc cargo rustfmt clippy rust-analyzer
    nixpkgs.rust-bin.nightly.latest.default
    nixpkgs.rust-analyzer

    # Python
    python3
    python3Packages.numpy
    python3Packages.python-lsp-server
    python3Packages.pylsp-mypy
    black
  ];

  RUST_BACKTRACE=1;
}
