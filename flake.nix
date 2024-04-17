{
  description = "Rust development flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override { extensions = [ "rust-src" ]; });
        darwinInputs = with pkgs;
          lib.optionals stdenv.isDarwin
            (with pkgs.darwin.apple_sdk.framework; [ ]);
      in
      with pkgs; {
        devShells.default = mkShell {
          packages = [ rust rust-analyzer-unwrapped ] ++ darwinInputs;
        };
        packages.default = {
            yabai-utils = rustPlatform.buildRustPackage {
                pname = "yabai-utils";
                version = "1.0.0";
                src = ./.;

                cargoSha256 = lib.fakeSha256;

                buildInputs = [];

                };
            };
      });
}

