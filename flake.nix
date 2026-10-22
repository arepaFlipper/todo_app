{
  description = "Rust and Trunk development environment";

  inputs = {
    nixpkgs.url = "github:Nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # Use the Rust overlay to get specific toolchain versions easily
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        # Define the exact Rust toolchain you need
        rustToolchain = pkgs.rust-bin.stable."1.96.1".default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "wasm32-unknown-unknown" ]; # Essential for Trunk/Yew
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rustToolchain
            pkgs.trunk      # Pulls Trunk from nixpkgs
            
            # Common dependencies often needed by Trunk/Wasm projects
            pkgs.wasm-bindgen-cli
            pkgs.binaryen   # Provides wasm-opt
            pkgs.tailwindcss_4  # Used by Trunk's tailwind-css asset
            pkgs.nodejs     # Provides npx for the Vercel CLI (npx vercel)
            pkgs.bun        # Installs the Vercel Claude plugin's packages
          ];

          env = {
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };

          shellHook = ''
            echo "⚡ Nix development environment loaded!"
            echo "rustc version: $(rustc --version)"
            echo "trunk version: $(trunk --version)"
          '';
        };
      });
}

