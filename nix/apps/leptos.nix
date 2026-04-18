{ craneLib, pkgs, src }:

let
  
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
      tailwindcss
    ];
    
    buildInputs = with pkgs; [
      openssl
    ];

    cargoExtraArgs = "--target wasm32-unknown-unknown";
  };

  # Build the artifacts
  # Note: Trunk usually downloads things, so we need to ensure everything is vendored or pre-built.
  # For now, we'll use a standard crane build but override the build command.
in
craneLib.buildPackage (commonArgs // {
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  buildPhaseCargoCommand = ''
    export HOME=$TMPDIR
    export TRUNK_SKIP_VERSION_CHECK=true
    mkdir -p node_modules/.bin
    ln -sf ${pkgs.tailwindcss}/bin/tailwindcss node_modules/.bin/tailwindcss
    trunk build --release --public-url "/" --skip-version-check
  '';

  installPhaseCommand = ''
    mkdir -p $out/share/nginx/html
    cp -r dist/* $out/share/nginx/html/
  '';
})
