{
  pkgs,
  inputs,
  ...
}: let
  inherit (inputs.self.lib.${pkgs.stdenv.hostPlatform.system}) craneLib commonArgs;

  cargoArtifacts = inputs.self.packages.${pkgs.stdenv.hostPlatform.system}.workspace-coverage-deps;
in
  craneLib.cargoTarpaulin (commonArgs
    // {
      inherit cargoArtifacts;
      pname = "workspace";
      # cargoLlvmCovExtraArgs = "--lcov --output-path $out";
      cargoTarpaulinExtraArgs = "--skip-clean --out lcov --output-dir $out --fail-under 60 --engine llvm";
      # cargoTarpaulinExtraArgs = "--print-rust-flags --skip-clean --out lcov --output-dir $out --fail-under 60 --engine llvm";
    })
