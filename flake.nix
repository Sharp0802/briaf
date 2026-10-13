{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      output = system:
        let
          overlays = [ rust-overlay.overlays.default ];
          pkgs = import nixpkgs { inherit system overlays; };
        in {
          devShells.${system}.default = pkgs.mkShell {
            packages = with pkgs; [
              cargo-deny
              cargo-llvm-cov
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            ];
          };
        };
    in
      builtins.foldl' (a: x: a // x) {} (map output systems);
}
