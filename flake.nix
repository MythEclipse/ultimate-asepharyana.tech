{
  description = "Ultimate Asepharyana Tech Monorepo - Nix Flake Migration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";

    # App Submodules as Inputs
    app-rust = { url = "github:MythEclipse/ultimate-asepharyana-tech-rust"; flake = false; };
    app-elysia = { url = "github:MythEclipse/ultimate-asepharyana-tech-elysia"; flake = false; };
    app-leptos = { url = "github:MythEclipse/ultimate-asepharyana-tech-leptos"; flake = false; };
    app-solidjs = { url = "github:MythEclipse/ultimate-asepharyana-tech-solidjs"; flake = false; };
    app-visuals = { url = "github:MythEclipse/ultimate-asepharyana-tech-visuals"; flake = false; };
    app-nextjs = { url = "github:MythEclipse/ultimate-asepharyana-tech-nextjs"; flake = false; };
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, systems, rust-overlay, crane, process-compose-flake, app-rust, app-elysia, app-leptos, app-solidjs, app-visuals, app-nextjs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        process-compose-flake.flakeModule
      ];
      
      systems = import systems;

      perSystem = { config, self', inputs', pkgs, system, ... }: 
        let
          rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            targets = [ "wasm32-unknown-unknown" ];
          };
          
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          
          # Applications
          apps-packages = {
            rust-backend = import ./nix/apps/rust.nix { inherit craneLib pkgs; src = app-rust; };
            leptos-frontend = import ./nix/apps/leptos.nix { inherit craneLib pkgs; src = app-leptos; };
            visuals = import ./nix/apps/visuals.nix { inherit craneLib pkgs; src = app-visuals; };
            elysia = import ./nix/apps/elysia.nix { inherit pkgs; src = app-elysia; };
            nextjs = import ./nix/apps/nextjs.nix { inherit pkgs; src = app-nextjs; };
            solidjs = import ./nix/apps/solidjs.nix { inherit pkgs; src = app-solidjs; };
          };
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          let
            solidjsSupervisorConfig = pkgs.runCommand "solidjs-supervisord-conf" {} ''
              mkdir -p $out/etc
              cat > $out/etc/supervisord.conf <<'EOF'
[supervisord]
nodaemon=true
user=appuser
logfile=/dev/null
logfile_maxbytes=0

[program:bun]
command=${pkgs.bun}/bin/bun run ${apps-packages.solidjs}/share/solidjs/.output/server/index.mjs
directory=/app
autostart=true
autorestart=true
stdout_logfile=/dev/stdout
stdout_logfile_maxbytes=0
stderr_logfile=/dev/stderr
stderr_logfile_maxbytes=0

[program:nginx]
command=${pkgs.nginx}/bin/nginx -g "daemon off;"
autostart=true
autorestart=true
stdout_logfile=/dev/stdout
stdout_logfile_maxbytes=0
stderr_logfile=/dev/stderr
stderr_logfile_maxbytes=0
EOF
            '';

            solidjsNginxConfig = pkgs.runCommand "solidjs-nginx-config" {} ''
              mkdir -p $out/etc/nginx/conf.d
              cp ${./infra/nginx/solidjs.conf} $out/etc/nginx/conf.d/default.conf
            '';

            leptosStatic = pkgs.runCommand "leptos-static" {} ''
              mkdir -p $out/usr/share/nginx/html
              cp -r ${apps-packages."leptos-frontend"}/share/nginx/html/* $out/usr/share/nginx/html/
            '';

            leptosNginxConfig = pkgs.runCommand "leptos-nginx-config" {} ''
              mkdir -p $out/etc/nginx/conf.d
              cp ${./infra/nginx/leptos.conf} $out/etc/nginx/conf.d/default.conf
            '';

            visualsStatic = pkgs.runCommand "visuals-static" {} ''
              mkdir -p $out/usr/share/nginx/html
              cp -r ${apps-packages.visuals}/share/nginx/html/* $out/usr/share/nginx/html/
            '';

            visualsNginxConfig = pkgs.runCommand "visuals-nginx-config" {} ''
              mkdir -p $out/etc/nginx/conf.d
              cp ${./infra/nginx/visuals.conf} $out/etc/nginx/conf.d/default.conf
            '';
          in
          packages = apps-packages // {
            default = pkgs.lib.mkForce apps-packages.rust-backend;
            services = config.process-compose.default.outputs.package;

            docker-rust = pkgs.dockerTools.buildLayeredImage {
              name = "rust-api";
              tag = "latest";
              contents = [ apps-packages.rust-backend pkgs.cacert ];
              config = {
                Cmd = [ "${apps-packages.rust-backend}/bin/rustexpress" ];
                ExposedPorts = { "4091/tcp" = {}; };
              };
            };

            docker-elysia = pkgs.dockerTools.buildLayeredImage {
              name = "elysia-api";
              tag = "latest";
              contents = [ pkgs.bun apps-packages.elysia pkgs.cacert ];
              config = {
                Cmd = [ "${pkgs.bun}/bin/bun" "run" "${apps-packages.elysia}/lib/dist/index.js" ];
                ExposedPorts = { "4092/tcp" = {}; };
              };
            };

            docker-nextjs = pkgs.dockerTools.buildLayeredImage {
              name = "nextjs-web";
              tag = "latest";
              contents = [ pkgs.nodejs apps-packages.nextjs pkgs.cacert ];
              config = {
                Cmd = [ "${pkgs.nodejs}/bin/node" "${apps-packages.nextjs}/share/nextjs/server.js" ];
                Env = [ "HOSTNAME=0.0.0.0" "PORT=3000" "NODE_ENV=production" "NEXT_TELEMETRY_DISABLED=1" ];
                ExposedPorts = { "3000/tcp" = {}; };
              };
            };

            docker-solidjs = pkgs.dockerTools.buildLayeredImage {
              name = "solidjs-web";
              tag = "latest";
              contents = [ pkgs.bun pkgs.nginx pkgs.supervisor apps-packages.solidjs solidjsSupervisorConfig solidjsNginxConfig pkgs.cacert ];
              config = {
                Cmd = [ "${pkgs.supervisor}/bin/supervisord" "-c" "/etc/supervisord.conf" ];
                ExposedPorts = { "80/tcp" = {}; };
              };
            };

            docker-leptos = pkgs.dockerTools.buildLayeredImage {
              name = "leptos-web";
              tag = "latest";
              contents = [ pkgs.nginx leptosStatic leptosNginxConfig pkgs.cacert ];
              config = {
                Cmd = [ "${pkgs.nginx}/bin/nginx" "-g" "daemon off;" ];
                ExposedPorts = { "80/tcp" = {}; };
              };
            };

            docker-visuals = pkgs.dockerTools.buildLayeredImage {
              name = "visuals";
              tag = "latest";
              contents = [ pkgs.nginx visualsStatic visualsNginxConfig pkgs.cacert ];
              config = {
                Cmd = [ "${pkgs.nginx}/bin/nginx" "-g" "daemon off;" ];
                ExposedPorts = { "80/tcp" = {}; };
              };
            };
          };

          devShells.default = pkgs.mkShell {
            name = "ultimate-asepharyana-dev";
            nativeBuildInputs = with pkgs; [
              rustToolchain
              bun
              nodejs_22
              pkg-config
              openssl
              trunk
              wasm-bindgen-cli
              binaryen
              process-compose
              mysql84
              redis
              minio-client
              gh
              git
            ];

            shellHook = ''
              export RUST_BACKTRACE=1
              export MYSQL_UNIX_PORT=$PWD/.nix/mysql.sock
              echo "🚀 Ultimate Asepharyana Tech Development Environment"
              echo "Layanan tersedia: MySQL (3306), Redis (6379), Minio (9000)"
              echo "Gunakan 'nix run .#services' untuk menjalankan seluruh stack."
            '';
          };

          process-compose.default = {
            settings.processes = {
              backend-rust = {
                command = "DATABASE_URL=${"$"}{DATABASE_URL:-mysql://root@127.0.0.1:3306/sosmed} JWT_SECRET=${"$"}{JWT_SECRET:-please-change-me-rust-jwt-secret-32-chars} REDIS_URL=${"$"}{REDIS_URL:-redis://127.0.0.1:6379} ${apps-packages.rust-backend}/bin/rustexpress";
              };
              backend-elysia = {
                command = "cd ${apps-packages.elysia}/lib && PORT=${"$"}{PORT:-4092} HOST=0.0.0.0 DATABASE_URL=${"$"}{DATABASE_URL:-mysql://root@127.0.0.1:3306/sosmed} JWT_SECRET=${"$"}{JWT_SECRET:-please-change-me-elysia-jwt-secret-32-chars} REDIS_URL=${"$"}{REDIS_URL:-redis://127.0.0.1:6379} MINIO_ENDPOINT=${"$"}{MINIO_ENDPOINT:-127.0.0.1} MINIO_PORT=${"$"}{MINIO_PORT:-9000} MINIO_USE_SSL=${"$"}{MINIO_USE_SSL:-false} MINIO_ACCESS_KEY=${"$"}{MINIO_ACCESS_KEY:-admin} MINIO_SECRET_KEY=${"$"}{MINIO_SECRET_KEY:-mytheclipse} MINIO_BUCKET_NAME=${"$"}{MINIO_BUCKET_NAME:-api} MINIO_PUBLIC_URL=${"$"}{MINIO_PUBLIC_URL:-http://127.0.0.1:9000} ${pkgs.bun}/bin/bun run dist/index.js";
              };
              
              frontend-nextjs = {
                command = "cd ${apps-packages.nextjs}/share/nextjs && HOSTNAME=0.0.0.0 PORT=${"$"}{NEXTJS_PORT:-3001} ${pkgs.nodejs}/bin/node server.js";
              };
              frontend-solidjs = {
                command = "cd ${apps-packages.solidjs}/share/solidjs && HOST=0.0.0.0 PORT=${"$"}{SOLIDJS_PORT:-3010} ${pkgs.bun}/bin/bun .output/server/index.mjs";
              };
            };
          };
        };
    };
}
