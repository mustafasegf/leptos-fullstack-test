{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          nativeBuildInputs = [ pkgs.bashInteractive ];

          NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [
            stdenv.cc.cc
            openssl
          ];

          NIX_LD = pkgs.runCommand "ld.so" {} ''
            ln -s "$(cat '${pkgs.stdenv.cc}/nix-support/dynamic-linker')" $out
          '';

          buildInputs = [
            nodePackages.prisma
            openssl
            pkg-config
            stdenv.cc.cc.lib
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          ];
          shellHook = with pkgs; ''
            export PRISMA_MIGRATION_ENGINE_BINARY="${prisma-engines}/bin/migration-engine"
            export PRISMA_QUERY_ENGINE_BINARY="${prisma-engines}/bin/query-engine"
            export PRISMA_QUERY_ENGINE_LIBRARY="${prisma-engines}/lib/libquery_engine.node"
            export PRISMA_INTROSPECTION_ENGINE_BINARY="${prisma-engines}/bin/introspection-engine"
            export PRISMA_FMT_BINARY="${prisma-engines}/bin/prisma-fmt"
          '';

        };
      }
    );
}
