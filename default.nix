{ pkgs ? import <nixpkgs> {}, }:

pkgs.mkShell {
  name = "modding-environment";

  buildInputs = with pkgs; [
    rustup
  ];
  shellHook = ''
    rustup default stable
    echo "Rust installed"
  '';
}
