{
  self,
  pkgs,
  lib,
  inputs,
  ...
}: rec {
  default = tok;
  tok = pkgs.callPackage ./tok.nix {inherit pkgs inputs lib self;};
}
