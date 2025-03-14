{ rust-analyzer-nightly }:
{
  programs = {
    nixfmt.enable = true;
    rust-analyzer = {
      edition = "2021";
      enable = true;
      package = rust-analyzer-nightly;
    };
    taplo.enable = true;
  };
  settings.formatter.nixfmt.excludes = [
    "_sources/generated.nix"
  ];
}
