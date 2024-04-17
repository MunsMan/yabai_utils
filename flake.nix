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
      in
      with pkgs; {
        devShells.default = mkShell {
          packages = [ rust rust-analyzer-unwrapped ];
        };
        packages.default = rustPlatform.buildRustPackage {
          pname = "yabai-utils";
          version = "1.0.0";
          src = ./.;

          cargoSha256 = "sha256-TPMXEJtrepZyA4efUQYa6i0SWglgSNEckOo34QOoOCU=";

          buildInputs = [ ];

        };
      });
}

