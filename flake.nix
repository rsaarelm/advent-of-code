{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    naersk.url = "github:nmattia/naersk/master";
  };

  outputs = { self, nixpkgs, utils, pre-commit-hooks, naersk, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pname = "advent-of-code";
        pkgs = import nixpkgs { inherit system; };
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

        apps.default = utils.lib.mkApp { drv = packages.default; };

        devShell = with pkgs;
          mkShell {
            buildInputs = [
              just
              tokei
              pre-commit

              # Rust
              cargo
              rustc
              rustfmt
              rust-analyzer
              clippy
              cargo-outdated

              # Python
              # python3
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
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            RUST_BACKTRACE = "1";
          };
      });
}
