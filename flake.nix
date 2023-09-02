{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # for completion
    flake-compat = {
      url = "github:inclyc/flake-compat";
      flake = false;
    };
  };
  outputs = inputs@{ flake-parts, crane, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      debug = true;
      systems = [
        "x86_64-linux"
      ];
      perSystem = { self', pkgs, system, ... }:
        let
          inherit (pkgs) lib;
          inherit (lib) importTOML;
          Cargo-toml = importTOML ./Cargo.toml;

          pname = "mentoriabot";
          version = Cargo-toml.workspace.package.version;

          # nightly rustfmt
          rustfmtNightly = pkgs.rustfmt.override { asNightly = true; };

          # crane config
          # see https://github.com/ipetkov/crane/blob/master/examples/trunk-workspace/flake.nix

          craneLib = crane.lib.${system}.overrideScope' (
            _final: _prev:
              {
                # ensure crane uses nightly rustfmt
                rustfmt = rustfmtNightly;
              }
          );

          # crate code source: keep only rust files and locales
          src = lib.cleanSourceWith {
            src = ./.; # The original, unfiltered source

            # keep only locales and code
            filter = path: type:
              # Keep locales
              (lib.hasInfix "/locales/" path) ||
              # Default filter from crane (allow .rs files)
              (craneLib.filterCargoSources path type);
          };

          commonCraneArgs = {
            inherit src pname version;

            buildInputs = [ pkgs.openssl ];
          };

          nativeCraneArgs = commonCraneArgs // {
            nativeBuildInputs = [ pkgs.pkg-config ];
          };

          # derivation with just the dependencies, so we don't have to
          # keep re-building them
          cargoArtifacts = craneLib.buildDepsOnly nativeCraneArgs;

          # our main derivation
          mentoriabot = craneLib.buildPackage (nativeCraneArgs // {
            inherit cargoArtifacts;
          });

        in
        {
          checks = {
            # ensure 'nix flake check' builds our crate
            inherit mentoriabot;

            # run clippy check on a separate derivation
            # error on warnings
            mentoriabot-clippy = craneLib.cargoClippy (nativeCraneArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

            # formatting check with nightly rustfmt
            mentoriabot-fmt = craneLib.cargoFmt commonCraneArgs;
          };

          # our main package (the bot)
          packages.default = mentoriabot;

          # "nix run" should run the binary in the default package
          # (aka start the bot!)
          apps.default = {
            type = "app";
            program = "${self'.packages.default}/bin/${pname}";
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              clippy
              rustfmtNightly
            ];

            buildInputs = with pkgs; [ openssl ];
            nativeBuildInputs = with pkgs; [ pkg-config ];
          };

          formatter = pkgs.nixpkgs-fmt;
        };
    };
}
