{ pkgs, src }:

let
  nodeDeps = pkgs.stdenv.mkDerivation {
    name = "solidjs-deps.tar.gz";
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
    outputHash = "sha256-z0KyUbRXwaLCWYvxoWjKFbOAhtOCl799+7qt9Sig3OA=";
  };
in

pkgs.stdenv.mkDerivation {
  pname = "solidjs-app";
  version = "0.1.0";
  inherit src;

  nativeBuildInputs = [ pkgs.bun pkgs.nodejs pkgs.gnutar ];

  buildPhase = ''
    export HOME=$TMPDIR
    tar -xzf ${nodeDeps}
    patchShebangs ./node_modules/.bin
    bun ./node_modules/vinxi/bin/cli.mjs build
  '';

  installPhase = ''
    mkdir -p $out/share/solidjs
    if [ -d .output ]; then
      cp -r .output $out/share/solidjs/
    fi
    if [ -d .vinxi ]; then
      cp -r .vinxi $out/share/solidjs/
    fi
  '';
}
