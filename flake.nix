{
  description = ''
A GameBoy emulator.
'';

  inputs = {
    # Requires unstable in order to build as of 2021-11-19
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachSystem [
      "x86_64-darwin"
      "x86_64-linux"
    ]
      (system:
        let
          cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));
          name = cargoConfig.package.name;
          pkgs = nixpkgs.legacyPackages.${system};
          additionalBuildInputs = with pkgs; [];
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
            inherit (fenix.packages.${system}.minimal) cargo rustc;
          }).buildRustPackage {
            pname = cargoConfig.package.name;
            version = cargoConfig.package.version;
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildFeatures = [];
            buildInputs = additionalBuildInputs;

            cargoSha256 = "sha256-fw/zUbYynrpeLGQ/uhs3LEq7tnECvatNAuDCJuCQGms=";
          };

          defaultPackage = self.packages.${system}.${name};

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [ cargo rustc rust-analyzer bat ] ++ additionalBuildInputs;

            shellHook = ''
              alias cat=bat
              export RUST_LOG=debug
            '';
          };
        });
}
