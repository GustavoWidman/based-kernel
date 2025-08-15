{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [
            "x86_64-unknown-none"
          ];
        };

        grub-mkrescue = pkgs.writeShellScriptBin "grub-mkrescue" ''
          #!${pkgs.runtimeShell}
          docker run --rm -v "$(pwd)":/work -w /work r3dlust/grub2 grub-mkrescue "$@"
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkgsCross.gnu64.buildPackages.binutils
            grub-mkrescue
            nushell
            qemu
          ];
        };
      }
    );
}
