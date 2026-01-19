{
  pkgs,
  inputs,
  ...
}: let
  persona-cli = inputs.self.packages.${pkgs.stdenv.hostPlatform.system}.persona-cli;
in
  pkgs.stdenv.mkDerivation {
    name = "validate-skills";
    src = pkgs.lib.fileset.toSource {
      root = ../..;
      fileset = pkgs.lib.fileset.unions [
        ../../.agent
        ../../AGENTS.md
      ];
    };

    nativeBuildInputs = [persona-cli];

    dontBuild = true;

    installPhase = ''
      mkdir -p $out
      persona check
      touch $out/pass
    '';
  }
