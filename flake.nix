{
  description = ''
An Rock, Paper, Scissors CLI
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
    flake-utils.lib.eachDefaultSystem (system:
      let
        cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));

        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs { inherit system overlays; };

        app = (pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        }).buildRustPackage {
          pname = cargoConfig.package.name;
          version = cargoConfig.package.version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          doCheck = true;

          cargoSha256 = "sha256-fw/zUbYynrpeLGQ/uhs3LEq7tnECvatNAuDCJuCQGms=";
        };
      in
        {
          packages.${app.pname} = app;
          packages.docker = pkgs.dockerTools.buildLayeredImage {
            name = app.pname;
            tag = app.version;
            config = {
              Entrypoint = [ "${app}/bin/${app.pname}" ];
            };
          };

          defaultPackage = self.packages.${system}.${app.pname};

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              pkg-config
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                ];
              })
              rust-analyzer
            ];

            RUST_LOG = "debug";
          };
        });
}
