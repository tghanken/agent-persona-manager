# {
#   pkgs,
#   inputs,
#   ...
# }: let
#   inherit (inputs.self.lib.${pkgs.stdenv.hostPlatform.system}) craneLib commonArgs;
# in
#   craneLib.cargoAudit (commonArgs
#     // {
#       name = "workspace-audit";
#       advisory-db = inputs.advisory-db;
#     })
#
# Above code fails without access to cargo-audit v0.22.0, use placeholder
{pkgs, ...}:
pkgs.stdenv.mkDerivation {
  name = "workspace-audit";
  src = ./.;
  installPhase = ''
    mkdir -p $out
  '';
}
