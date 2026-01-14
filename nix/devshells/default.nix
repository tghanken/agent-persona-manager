{
  pkgs,
  flake,
  system,
  perSystem,
  ...
}: let
  inherit (flake.lib.${system}) craneLib;
in
  craneLib.devShell {
    packages = [
      pkgs.nix-fast-build
      pkgs.nix-output-monitor

      # Cargo tools
      pkgs.cargo-nextest
      pkgs.cargo-tarpaulin
      perSystem.nixpkgs-unstable.cargo-audit
      perSystem.nixpkgs-unstable.cargo-rail

      # Workspace deps
      flake.packages.${system}.workspace-deps
    ];
  }
