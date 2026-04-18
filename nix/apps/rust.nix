{ craneLib, pkgs, src }:

let
  # Pre-fetch the Swagger UI zip via Nix (goes into Nix store, read-only, mode 444)
  swagger-zip = pkgs.fetchurl {
    url = "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v5.17.14.zip";
    sha256 = "481244d0812097b11fbaeef79f71d942b171617f9c9f9514e63acbe13e71ccdc";
  };
in

craneLib.buildPackage {
  pname = "rust-backend";
  version = "0.1.0";
  inherit src;
  strictDeps = true;

  buildInputs = with pkgs; [
    openssl
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
    unzip # utoipa-swagger-ui build script may need to extract the zip
  ];

  # preBuild runs inside the writable /build sandbox.
  # The key insight: `cat ... >` creates a fresh file with default 644 permissions,
  # unlike `cp` which would inherit the 444 read-only mode from the Nix store.
  # Rust's fs::copy() internally calls fchmod() to mirror source permissions onto the
  # destination, which is what caused the "Permission denied" panic at build.rs:180.
  # By providing a 644 file, fchmod() will succeed inside the Nix sandbox.
  preBuild = ''
    cat ${swagger-zip} > $NIX_BUILD_TOP/swagger-ui.zip
    export SWAGGER_UI_DOWNLOAD_URL="file://$NIX_BUILD_TOP/swagger-ui.zip"
  '';
}
