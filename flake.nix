{
  inputs = {
    esp32-rust = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:svelterust/esp32";
    };
    fenix = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:nix-community/fenix";
    };
    flake-parts = {
      inputs.nixpkgs-lib.follows = "nixpkgs";
      url = "github:hercules-ci/flake-parts";
    };
    git-hooks = {
      inputs = {
        flake-compat.follows = "";
        nixpkgs.follows = "nixpkgs";
      };
      url = "github:cachix/git-hooks.nix";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.11";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:numtide/treefmt-nix";
    };
  };
  outputs =
    inputs@{
      fenix,
      flake-parts,
      git-hooks,
      nixpkgs,
      systems,
      treefmt-nix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        git-hooks.flakeModule
        treefmt-nix.flakeModule
      ];
      perSystem =
        { inputs', system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              fenix.overlays.default
            ];
          };
          sources = import ./_sources/generated.nix {
            inherit (pkgs)
              dockerTools
              fetchFromGitHub
              fetchgit
              fetchurl
              ;
          };
        in
        {
          _module.args.pkgs = pkgs;
          devShells.default =
            let
              esp32-rust = inputs'.esp32-rust.packages.esp32.overrideAttrs (
                _: _: {
                  inherit (sources.idf-rust) src;
                }
              );
            in
            pkgs.mkShell {
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
                with pkgs;
                [
                  libxml2
                  stdenv.cc.cc.lib
                  zlib
                ]
              );
              packages =
                (with pkgs; [
                  ldproxy
                  nvfetcher
                ])
                ++ (with pkgs.fenix; [
                  (combine [
                    latest.toolchain
                    targets.thumbv7em-none-eabi.latest.rust-std
                    targets.thumbv7em-none-eabihf.latest.rust-std
                  ])
                ]);
              shellHook = ''
                export PATH="${esp32-rust}/.rustup/toolchains/esp/bin:$PATH"
                export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
              '';
            };
          pre-commit = {
            check.enable = true;
            settings = {
              hooks = {
                actionlint.enable = true;
                check-json.enable = true;
                check-toml.enable = true;
                editorconfig-checker = {
                  enable = true;
                  excludes = [
                    "Cargo.lock"
                    "flake.lock"
                  ];
                };
                luacheck.enable = true;
                markdownlint.enable = true;
                yamlfmt.enable = true;
                yamllint.enable = true;
              };
              src = ./.;
            };
          };
          treefmt = import ./treefmt.nix;
        };
      systems = import systems;
    };
}
