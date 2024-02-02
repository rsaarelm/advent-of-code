{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nmattia/naersk/master";
  };

  outputs = { self, nixpkgs, flake-utils, pre-commit-hooks, rust-overlay, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pname = "advent-of-code";
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust = ((pkgs.rustChannelOf { channel = "nightly"; }).default.override {
          extensions = [ "rust-src" ];
        });
        naersk-lib = pkgs.callPackage naersk { };
      in rec {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = { nixpkgs-fmt.enable = true; };
          };
        };

        # Graphical version
        packages.default = naersk-lib.buildPackage { src = ./.; };

        apps.default = flake-utils.lib.mkApp { drv = packages.default; };

        devShell = with pkgs;
          mkShell {
            buildInputs = [
              just
              tokei
              pre-commit
              graphviz

              # Rust
              rust
              rust-analyzer
              clippy
              cargo-outdated
              cargo-udeps

              # Needed for rust-Z3
              clang.cc.lib
              z3.dev

              # Python
              python3
              python3Packages.z3
              # python3Packages.numpy
              # python3Packages.python-lsp-server
              # # FIXME 2022-11-30 Broken package?
              # #python3Packages.pylsp-mypy
              # black

              # Clojure
              # clojure
              # jdk11_headless
              # clojure-lsp

              # Zig
              # zig

              # Idris
              idris2

              # Nushell
              nushell
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            RUST_BACKTRACE = "1";
            LD_LIBRARY_PATH = with pkgs;
              pkgs.lib.makeLibraryPath [
                libclang
              ];
          };
      });
}
