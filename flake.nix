{
  description = "Uiua LLVM codegen backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };
        devShells.default = pkgs.mkShell.override {
          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
        } {
          packages = with pkgs; [
              (rust-bin.stable.latest.complete)
              llvm_22.dev
            ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            libxml2
            libffi
          ];
            # LLVM_SYS_221_PREFIX="${pkgs.llvmPackages_22.llvm.dev}";
        };
      };
    };
}
