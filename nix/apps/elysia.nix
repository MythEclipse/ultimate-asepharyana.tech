{ pkgs, src }:

let
  # Fixed-output derivation yang menghasilkan FLAT tarball (bukan directory).
  # outputHashMode = "flat" berarti Nix hanya hash file $out itu sendiri,
  # sehingga referensi Nix store path di dalam node_modules tidak jadi masalah.
  nodeDeps = pkgs.stdenv.mkDerivation {
    name = "elysia-deps.tar.gz";
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
    outputHash = "sha256-NFXKdl/WmQeHSaPdU0OBIjda1zhAdEkNHiLGW4rXINI=";
  };
in

pkgs.stdenv.mkDerivation {
  pname = "elysia-app";
  version = "0.1.0";
  inherit src;

  nativeBuildInputs = [ pkgs.bun pkgs.nodejs pkgs.gnutar ];

  buildPhase = ''
    export HOME=$TMPDIR
    tar -xzf ${nodeDeps}
    patchShebangs ./node_modules/.bin
    bun run build
  '';

  installPhase = ''
    mkdir -p $out/lib
    cp -r dist $out/lib/
    cp package.json $out/lib/
    cp -r node_modules $out/lib/
  '';
}
