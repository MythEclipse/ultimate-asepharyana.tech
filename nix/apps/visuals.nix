{ craneLib, pkgs, src }:

let
  
  commonArgs = {
    pname = "visuals";
    version = "0.1.0";
    inherit src;
    strictDeps = true;
    
    nativeBuildInputs = with pkgs; [
      pkg-config
      trunk
      wasm-bindgen-cli
      binaryen
    ];
    
    buildInputs = with pkgs; [
      openssl
    ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.Security
    ];

    cargoExtraArgs = "--target wasm32-unknown-unknown";
  };
in
craneLib.buildPackage (commonArgs // {
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  buildPhaseCargoCommand = ''
    export HOME=$TMPDIR
    export TRUNK_SKIP_VERSION_CHECK=true
    wasm-bindgen --version
    trunk build --release --skip-version-check
  '';

  installPhaseCommand = ''
    mkdir -p $out/share/visuals
    cp -r dist/* $out/share/visuals/
  '';
})
