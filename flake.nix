{
  description = "find and remove `target` directories and save some disk space";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-22.11";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs { inherit system; });
        toolchain = with fenix.packages.${system};combine [ stable.rustc stable.cargo ];
        rustPlatform = pkgs.makeRustPlatform { rustc = toolchain; cargo = toolchain; };
        drv = pkgs.callPackage (import ./default.nix) { inherit rustPlatform; };
      in
      {
        packages.default = drv;
      });
}
