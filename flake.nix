{
  inputs = {
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
      nixpkgs,
      systems,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./treefmt.nix
        ./git-hooks.nix
      ];
      perSystem =
        {
          inputs',
          pkgs,
          ...
        }:
        {
          devShells.default = pkgs.mkShell {
            packages = with inputs'.fenix.packages; [
              (combine [
                latest.toolchain
                targets.thumbv7em-none-eabi.latest.rust-std
                targets.thumbv7em-none-eabihf.latest.rust-std
              ])
            ];
            shellHook = ''
              export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
            '';
          };
        };
      systems = import systems;
    };
}
