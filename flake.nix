{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.crane.url = "github:ipetkov/crane"; # avoid rebuilding rust dependencies
  outputs = {
    self,
    nixpkgs,
    crane,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    basename = "ants";
  in {
    devShell.${system} = pkgs.mkShell {
      packages = with pkgs; [
        nushell
        nodejs_24
        wasm-bindgen-cli_0_2_100
        cargo
        rustc
        lld
      ];
      RUSTFLAGS = "--cfg getrandom_backend=\"wasm_js\"";
    };
    packages.${system}.default = let
      craneLib = crane.mkLib pkgs;
      wasm = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./wasm;
        strictDeps = true;
        doCheck = false;
        nativeBuildInputs = [pkgs.lld];
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        CARGO_BUILD_RUSTFLAGS = "--cfg getrandom_backend=\"wasm_js\"";
      };
      wasmGlue = pkgs.runCommand "wasm-glue" {buildInputs = [pkgs.wasm-bindgen-cli_0_2_100];} ''
        wasm-bindgen --no-typescript --target bundler --out-dir $out/src ${wasm}/lib/*.wasm
      '';
    in
      pkgs.buildNpmPackage {
        name = basename;
        src = ./app;
        npmDepsHash = "sha256-mx0Peu03lwJaWzeFZmaqsZuZ9UxkGfzW9LFg4H88YOs=";
        preBuild = ''
          cp --recursive ${wasmGlue}/* ./wasm
        '';
        installPhase = ''
          mkdir -p $out
          cp --recursive ./www/* $out/
        '';
      };
    apps.${system}.default = let
      testServer = pkgs.writeShellApplication {
        name = "${basename}-test-server";
        runtimeInputs = with pkgs; [static-web-server];
        text = ''
          echo "serving, visit http://localhost:8080 in a browser"
          static-web-server --port 8080 --root ${self.packages.${system}.default}
        '';
      };
    in {
      type = "app";
      program = "${testServer}/bin/${basename}-test-server";
    };
  };
}
