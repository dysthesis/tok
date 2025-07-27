{
  pkgs,
  self,
  ...
}:
pkgs.mkShell {
  name = "tok";

  packages = with pkgs; [
    nixd
    alejandra
    statix
    deadnix
    cargo
    rustToolchains.nightly
    bacon
    rust-analyzer
    cargo-audit
    cargo-nextest
    cargo-tarpaulin
    cargo-bloat
    cargo-expand
  ];
}
