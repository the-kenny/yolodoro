{ nixpkgs ? <nixpkgs> }:
let
  pkgs = import nixpkgs {};
in rec {
  rustEnv = pkgs.stdenv.mkDerivation {
    name = "rust";
    version = "1.2.3.4";
    src = ./.;
    buildInputs = with pkgs; [ rustc cargo xlibs.libX11 gcc pkgconfig dbus ];

    RUST_LOG="pomodoro=info";
    RUST_SRC_PATH="${pkgs.rustc.src}";

    shellHook = ''
      export PATH="target/debug/:$PATH";
    '';
  };
} 
