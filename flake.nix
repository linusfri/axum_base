{
  description = "Flake to handle dev env.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, ... }@inputs:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        buildDeps = [
          
        ];

        devDeps = buildDeps ++ [
          pkgs.nettools
        ];
      in
      {
        devShells.build = pkgs.mkShell { buildInputs = buildDeps; };
        devShells.default = pkgs.mkShell { buildInputs = devDeps; };
      });
}
