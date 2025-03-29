{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    { inputs', pkgs, ... }:
    {
      treefmt = {
        programs = {
          mdformat = {
            enable = true;
            package = pkgs.mdformat.withPlugins (
              ps: with ps; [
                mdformat-gfm
              ]
            );
            settings = {
              end-of-line = "lf";
              number = true;
              wrap = 80;
            };
          };
          nixfmt.enable = true;
          rust-analyzer = {
            edition = "2021";
            enable = true;
            package = inputs'.fenix.packages.rust-analyzer;
          };
          taplo.enable = true;
        };
        settings.formatter.nixfmt.excludes = [
          "_sources/generated.nix"
        ];
      };
    };
}
