{ craneLib, pkgs, src }:

let
  nodeDeps = pkgs.stdenv.mkDerivation {
    name = "leptos-deps.tar.gz";
    inherit src;
    nativeBuildInputs = [ pkgs.bun pkgs.cacert ];
    buildPhase = ''
      export HOME=$TMPDIR
      bun install --frozen-lockfile
    '';
    installPhase = ''
      find node_modules -exec touch -h -t 197001010000.00 {} +
      tar --sort=name --mtime='1970-01-01 00:00:00Z' --owner=0 --group=0 --numeric-owner -czf $out node_modules
    '';
    outputHashMode = "flat";
    outputHashAlgo = "sha256";
    outputHash = "sha256-5VnDSWhPICJ8khprSAUqfMGQImub8BM8IcA42QVbS0s=";
  };

  # Common arguments for crane
  commonArgs = {
    pname = "apps-leptos";
    version = "0.1.0";
    inherit src;
    strictDeps = true;
    
    nativeBuildInputs = with pkgs; [
      pkg-config
      trunk
      wasm-bindgen-cli
      binaryen # wasm-opt
      bun
      nodejs
      gnutar
      coreutils
    ];
    
    buildInputs = with pkgs; [
      openssl
    ];

    cargoExtraArgs = "--target wasm32-unknown-unknown";
  };

in
craneLib.buildPackage (commonArgs // {
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  buildPhaseCargoCommand = ''
    export HOME=$TMPDIR
    export TRUNK_SKIP_VERSION_CHECK=true
    export TRUNK_OFFLINE=true
    tar -xzf ${nodeDeps}
    export NODE_PATH=$PWD/node_modules
    export PATH=$PWD/node_modules/.bin:$PATH
    patchShebangs ./node_modules/.bin
    find ./node_modules/.bin -type f -exec sed -i '1s|^#!/usr/bin/env node|#!${pkgs.nodejs}/bin/node|' {} +
    wasm-bindgen --version
    trunk build --release --public-url "/" --skip-version-check
  '';

  installPhaseCommand = ''
    mkdir -p $out/share/nginx/html
    cp -r dist/* $out/share/nginx/html/
  '';

  passthru = { inherit nodeDeps; };
})
