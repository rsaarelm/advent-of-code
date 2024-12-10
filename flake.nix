{
  description = "Advent of Code dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            just
            tokei
            pre-commit
            graphviz
            # Needed by aoc tool
            pkg-config
            openssl

            rustc
            rust-analyzer
            clippy
            rust-script

            python3
            python3Packages.z3
          ];
        };
      }
    );
}
