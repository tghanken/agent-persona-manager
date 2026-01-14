{
  pkgs,
  inputs,
  ...
}: let
  inherit (inputs.self.lib.${pkgs.stdenv.hostPlatform.system}) craneLib commonArgs;

  cargoArtifacts = inputs.self.packages.${pkgs.stdenv.hostPlatform.system}.workspace-deps;
  pname = "persona-cli";
in
  craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts pname;
    })
