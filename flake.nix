{
  description = "devshell for osprey";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    dioxus.url = "github:j4ger/dioxus";
  };

  outputs = { nixpkgs, flake-utils, dioxus, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        baseShell = dioxus.devShells.${system}.web;
      in {
        devShells.default = pkgs.mkShell {
          name = "osprey-web-devshell";
          buildInputs = baseShell.buildInputs ++ [ ];

          shellHook = "\n";
        };
      });
}
