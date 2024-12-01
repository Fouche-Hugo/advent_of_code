{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    python314
    cargo
    clippy
    rustfmt
    rustc
    (vscode-with-extensions.override {
      vscodeExtensions = with vscode-extensions; [
        bbenoist.nix
        arrterian.nix-env-selector
        rust-lang.rust-analyzer
        tamasfe.even-better-toml
        vscodevim.vim
        vscode-extensions.ms-python.python
      ];
    })
  ];
}