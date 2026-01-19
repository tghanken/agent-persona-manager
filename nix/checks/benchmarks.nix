{
  pkgs,
  inputs,
  ...
}: let
  inherit (inputs.self.lib.${pkgs.stdenv.hostPlatform.system}) craneLib commonArgs;

  cargoArtifacts = inputs.self.packages.${pkgs.stdenv.hostPlatform.system}.workspace-deps;
in
  craneLib.mkCargoDerivation (commonArgs
    // {
      inherit cargoArtifacts;
      pname = "benchmarks";

      buildPhaseCargoCommand = "cargo bench --bench commands";

      installPhase = "touch $out";

      doCheck = false;
    })
