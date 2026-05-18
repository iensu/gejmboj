{
  description = ''
A GameBoy emulator.
'';

  inputs = {
    # Requires unstable in order to build as of 2021-11-19
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, rust-overlay, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));
          name = cargoConfig.package.name;
          version = cargoConfig.package.version;
          overlays = [(import rust-overlay)];
          pkgs = import nixpkgs { inherit system overlays; };
          rustBinaries = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          naersk-lib = pkgs.callPackage naersk {
            cargo = rustBinaries;
            rustc = rustBinaries;
          };
          package = naersk-lib.buildPackage {
            inherit name version;
            root = builtins.path { path = ./.; inherit name; };
          };
        in
        {
          apps.${name} = {
            type = "app";
            program = "${self.pkgs.${system}.${name}}/bin/${name}";
          };
          packages.${name} = package;
          defaultPackage = self.packages.${system}.${name};

          devShell = with pkgs; pkgs.mkShell {
            buildInputs = [
              rustBinaries
              rust-analyzer
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            RUST_LOG = "debug";
          };
        });
}
