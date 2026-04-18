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
      rust-bin.nightly.latest.default
    ];
    
    buildInputs = with pkgs; [
      openssl
    ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.Security
    ];
  };
in
craneLib.buildPackage (commonArgs // {
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  buildPhaseCargoCommand = ''
    export HOME=$TMPDIR
    trunk build --release
  '';

  installPhaseCommand = ''
    mkdir -p $out/share/visuals
    cp -r dist/* $out/share/visuals/
  '';
})
