{ craneLib, pkgs, src }:

let
  # Pre-fetch the Swagger UI zip via Nix (goes into Nix store, read-only, mode 444)
  swagger-zip = pkgs.fetchurl {
    url = "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.17.14.zip";
    sha256 = "481244d0812097b11fbaeef79f71d942b171617f9c9f9514e63acbe13e71ccdc";
  };

  commonArgs = {
    pname = "rustexpress";
    version = "0.1.0";
    strictDeps = true;

    buildInputs = with pkgs; [
      openssl
    ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    ];

    nativeBuildInputs = with pkgs; [
      pkg-config
      unzip
    ];

    # preBuild runs inside the writable /build sandbox.
    # By providing a 644 file, fchmod() will succeed inside the Nix sandbox.
    preBuild = ''
      cat ${swagger-zip} > $NIX_BUILD_TOP/swagger-ui.zip
      export SWAGGER_UI_DOWNLOAD_URL="file://$NIX_BUILD_TOP/swagger-ui.zip"
    '';
  };

  # Dependency build - only needs Cargo files
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    src = craneLib.cleanCargoSource src;
  });
in
craneLib.buildPackage (commonArgs // {
  inherit cargoArtifacts src;
})
