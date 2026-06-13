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
          # https://github.com/robert/gameboy-doctor
          # Single Python3 script + truth data; paths resolved relative to the
          # script via __file__, so the data must stay alongside it.
          gameboy-doctor = pkgs.stdenv.mkDerivation {
            pname = "gameboy-doctor";
            version = "unstable-cc74277";
            src = pkgs.fetchFromGitHub {
              owner = "robert";
              repo = "gameboy-doctor";
              rev = "cc742777af7d71de3061c65733ecfa250237012b";
              hash = "sha256-tZnH7gEMwi1TkgZITl0cHxm96FJ+0VWKQ/WtygrFyXs=";
            };
            nativeBuildInputs = [ pkgs.makeWrapper pkgs.unzip ];
            dontConfigure = true;
            dontBuild = true;
            installPhase = ''
              runHook preInstall
              mkdir -p $out/libexec/gameboy-doctor
              cp -r . $out/libexec/gameboy-doctor/
              chmod +x $out/libexec/gameboy-doctor/gameboy-doctor

              # The tool lazily unzips its truth data next to the script on first
              # run (see the unzip() function). In the read-only store that write
              # fails, so pre-extract everything here. With the .log files already
              # present and correctly sized, the runtime unzip() early-returns
              # without touching the filesystem.
              truth=$out/libexec/gameboy-doctor/truth
              for zip in "$truth"/zipped/*/*.zip; do
                rom_type=$(basename "$(dirname "$zip")")
                dest="$truth/unzipped/$rom_type"
                mkdir -p "$dest"
                unzip -o "$zip" -d "$dest"
              done
              makeWrapper $out/libexec/gameboy-doctor/gameboy-doctor \
                $out/bin/gameboy-doctor \
                --prefix PATH : ${pkgs.python3}/bin
              runHook postInstall
            '';
          };
        in
        {
          devShell = with pkgs; pkgs.mkShell {
            buildInputs = [
              rustBinaries
              rust-analyzer
              xxd
              gameboy-doctor
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            RUST_LOG = "debug";
          };
        });
}
