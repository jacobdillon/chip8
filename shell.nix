let
  unstable = import <unstable> {};
  pkgs = import <nixpkgs> {};
in
  with pkgs; mkShell {
    buildInputs = [
      gcc
      clang
      gdb
      valgrind
    ];
  }
