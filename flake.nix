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
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, flake-utils, rust-overlay, nixpkgs }:
    flake-utils.lib.eachSystem [
      "x86_64-darwin"
      "x86_64-linux"
    ]
      (system:
        let
          cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));
          name = cargoConfig.package.name;
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          additionalBuildInputs = with pkgs; [];
          enabledFeatures = [];
        in
        {
          checks.format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ rustfmt cargo ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            touch $out # success!
            '';

          apps.${name} = {
            type = "app";
            program = "${self.pkgs.${system}.${name}}/bin/${name}";
          };

          packages.${name} = (pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.minimal;
            rustc = pkgs.rust-bin.stable.latest.minimal;
          }).buildRustPackage {
            pname = cargoConfig.package.name;
            version = cargoConfig.package.version;
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildFeatures = enabledFeatures;
            buildInputs = additionalBuildInputs;

            doCheck = true;
            cargoTestFlags = "--workspace";

            cargoSha256 = "sha256-fw/zUbYynrpeLGQ/uhs3LEq7tnECvatNAuDCJuCQGms=";
          };

          defaultPackage = self.packages.${system}.${name};

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              openssl
              pkgconfig
              exa
              fd
              bat
              rust-bin.stable.latest.default
              rust-analyzer
            ] ++ additionalBuildInputs;

            shellHook = ''
              alias cat=bat
              alias ls=exa
              alias find=fd
              export RUST_LOG=debug
            '';
          };
        });
}
