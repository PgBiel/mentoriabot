{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-manifest = {
      # pin to rustc 1.70.0
      url = "https://static.rust-lang.org/dist/2023-06-01/channel-rust-1.70.0.toml";
      flake = false;
    };
  };
  outputs = inputs@{ flake-parts, fenix, rust-manifest, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
      ];
      perSystem = { pkgs, system, ... }:
        let
          inherit (pkgs) lib;
          inherit (lib) importTOML;
          Cargo-toml = importTOML ./Cargo.toml;
          toolchain = (fenix.packages.${system}.fromManifestFile rust-manifest)
            .minimalToolchain;
          rustPlatform = pkgs.makeRustPlatform {
            rustc = toolchain;
            cargo = toolchain;
          };
        in {
          packages.default = rustPlatform.buildRustPackage {
            pname = "mentoriabot";
            version = Cargo-toml.workspace.package.version;

            # these seem to be required
            buildInputs = with pkgs; [ openssl ];
            nativeBuildInputs = with pkgs; [ pkg-config ];

            src = ./.;

            # replace with 'lib.fakeHash' to get the new hash
            cargoHash = "sha256-VluDBd2axnUpkfcMFXFt7jdUWEk73dxvo4YMH80M9fA=";

            meta = with lib; {
              description = "Mentoria bot";
              homepage = "https://github.com/PgBiel/mentoriabot";
              license = licenses.mit;
              maintainers = [];
            };
          };

          formatter = pkgs.nixpkgs-fmt;
        };
    };
}
