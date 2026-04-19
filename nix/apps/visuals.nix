{ craneLib, pkgs, src }:

let
  
  commonArgs = {
    pname = "visuals";
    version = "0.1.0";
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
    doNotPostBuildInstallCargoBinaries = true;
    doCheck = false;
  };

  # Dependency build - only needs Cargo files
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    src = craneLib.cleanCargoSource src;
  });
in
craneLib.buildPackage (commonArgs // {
  inherit cargoArtifacts src;

  buildPhaseCargoCommand = ''
    export HOME=$TMPDIR
    export TRUNK_SKIP_VERSION_CHECK=true
    export TRUNK_OFFLINE=false
    wasm-bindgen --version
    trunk build --release --skip-version-check
  '';

  installPhaseCommand = ''
    mkdir -p $out/share/visuals
    cp -r dist/* $out/share/visuals/
  '';
})
