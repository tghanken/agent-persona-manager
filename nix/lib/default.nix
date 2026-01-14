{inputs, ...}: let
  eachSystem = inputs.nixpkgs.lib.genAttrs (import inputs.systems);
in
  eachSystem (
    system: let
      pkgs = inputs.nixpkgs.legacyPackages.${system};
      pkgsWithRust = pkgs.extend inputs.rust-overlay.overlays.default;
      rustToolchain = pkgsWithRust.rust-bin.stable."1.92.0".default.override {
        extensions = ["rust-src" "rust-std" "llvm-tools-preview"];
      };
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      workspaceSrc = pkgs.lib.cleanSourceWith {
        src = ../../.;
        filter = path: type: (craneLib.filterCargoSources path type);
      };

      commonArgs = {
        src = workspaceSrc;
        strictDeps = true;
        doCheck = false;
        buildInputs =
          [
            pkgs.pkg-config
            pkgs.openssl
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };
    in rec {
      inherit craneLib commonArgs;
      treefmt = inputs.treefmt-nix.lib.evalModule pkgs (import ./treefmt-config.nix);
    }
  )
