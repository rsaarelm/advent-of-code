with import <nixpkgs> {};
mkShell {
  buildInputs = [
    babashka just

    # Clojure
    clojure jdk11_headless clojure-lsp

    # Rust
    rustc cargo rustfmt rust-analyzer clippy

    # Python
    python3
  ];
}
