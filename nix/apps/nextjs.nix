{ pkgs, src }:

let
  nodeDeps = pkgs.stdenv.mkDerivation {
    name = "nextjs-deps.tar.gz";
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
    outputHash = "sha256-7YwOkZUu3e0UAwdSrki0sa41WF5KHVhHq/yqtViGzcs=";
  };
in

pkgs.stdenv.mkDerivation {
  pname = "nextjs-app";
  version = "0.1.0";
  inherit src;

  nativeBuildInputs = [ pkgs.bun pkgs.nodejs pkgs.gnutar ];

  buildPhase = ''
    export HOME=$TMPDIR
    export NEXT_TELEMETRY_DISABLED=1
    tar -xzf ${nodeDeps}
    patchShebangs ./node_modules/.bin
    node ./node_modules/next/dist/bin/next build
  '';

  installPhase = ''
    mkdir -p $out/share/nextjs
    cp -rv .next/standalone/. $out/share/nextjs/
    
    # Standalone mode expects .next/static and public to be manually copied
    mkdir -p $out/share/nextjs/.next/static
    if [ -d ".next/static" ]; then
      cp -rv .next/static/. $out/share/nextjs/.next/static/
    fi
    
    if [ -d "public" ]; then
      cp -rv public $out/share/nextjs/
    fi
  '';
}
