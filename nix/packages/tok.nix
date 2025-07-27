{
  self,
  lib,
  pkgs,
  rustPlatform,
  symlinkJoin,
  cargo,
  rustc,
  luajit,
  pkg-config,
  makeWrapper,
  buildEnv,
  ...
}:
  unwrapped = rustPlatform.buildRustPackage rec {
    name = "tok";
    version = "0.1.0";
    
    nativeBuildInputs = [
      cargo
      rustc
      pkg-config
    ];

    src = ../../.;
    cargoLock.lockFile = "${src}/Cargo.lock";
  }
