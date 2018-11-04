{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    RUST_LOG="api=debug";
    buildInputs = [pkgs.pkgconfig pkgs.openssl pkgs.postgresql.lib (pkgs.python3.withPackages (ps: with ps; [requests])) ]; 
    shellHook = ''
        export PATH="$HOME/.cargo/bin/:$PATH"
    '';
}
