{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    RUST_LOG="api=debug";
    buildInputs = with pkgs; [postgresql.lib (python3.withPackages (ps: with ps; [requests])) ]; 
    shellHook = ''
        export PATH="$HOME/.cargo/bin/:$PATH"
    '';
}
