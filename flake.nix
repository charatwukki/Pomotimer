{
  description = "Tauri v2 app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    # crane-tauri declares no inputs of its own, so there is nothing to follow
    # or override (the previous follows clause warned about non-existent inputs).
    crane-tauri.url = "github:JPHutchins/crane-tauri";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      crane-tauri,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        craneLib = crane.mkLib pkgs;

        frontend = pkgs.buildNpmPackage {
          pname = "pomotimer"; # TODO: change
          version = "0.1.0"; # TODO: change

          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./package.json
              ./package-lock.json
              ./vite.config.js
              ./index.html
              ./src
              ./public
            ];
          };

          npmDepsHash = "sha256-cjgpsBS/LF/9RVOR1I8IS+idyAC3bCf8SMQ7bhC1Hbk="; # TODO: build once to get correct hash

          installPhase = ''
            runHook preInstall
            cp -r dist $out
            runHook postInstall
          '';
        };

        tauri = crane-tauri.lib.buildTauriApp { inherit pkgs craneLib; } {
          pname = "pomotimer"; # TODO: change
          version = "0.1.0"; # TODO: change
          src = ./.;
          inherit frontend;
        };
      in
      {
        packages = {
          inherit frontend;
          default = tauri.app;
        };

        checks = {
          inherit (tauri) app;

          clippy = craneLib.cargoClippy (
            tauri.commonArgs
            // {
              cargoArtifacts = tauri.cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- -D warnings";
              TAURI_CONFIG = tauri.tauriConfig;
            }
          );

          fmt = craneLib.cargoFmt { src = tauri.commonArgs.src; };
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
        };
      }
    );
}
