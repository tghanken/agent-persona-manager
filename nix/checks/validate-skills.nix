{pkgs, ...}:
pkgs.stdenv.mkDerivation {
  name = "validate-skills";
  src = pkgs.lib.fileset.toSource {
    root = ../..;
    fileset = pkgs.lib.fileset.unions [
      ../../nix/scripts/validate_skills.py
      ../../.agent/skills
    ];
  };

  buildInputs = [pkgs.python3];

  dontBuild = true;

  installPhase = ''
    mkdir -p $out
    python3 nix/scripts/validate_skills.py
    touch $out/pass
  '';
}
