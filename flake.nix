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

          packages = apps-packages // {
            default = pkgs.lib.mkForce apps-packages.rust-backend;
            services = config.process-compose.default.outputs.package;
            
            # Docker Images built via Nix
            docker-rust = pkgs.dockerTools.buildLayeredImage {
              name = "rust-api";
              tag = "latest";
              contents = [ apps-packages.rust-backend pkgs.cacert ];
              config = {
                Cmd = [ "${apps-packages.rust-backend}/bin/rustexpress" ];
                ExposedPorts = { "8080/tcp" = {}; };
              };
            };
            
            docker-elysia = pkgs.dockerTools.buildLayeredImage {
              name = "elysia-api";
              tag = "latest";
              contents = [ pkgs.bun apps-packages.elysia ];
              config = {
                Cmd = [ "${pkgs.bun}/bin/bun" "run" "${apps-packages.elysia}/lib/index.ts" ];
                ExposedPorts = { "4092/tcp" = {}; };
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
              mysql = {
                command = ''
                  mkdir -p ./.nix ./.nix/data/mysql ./.nix/tmp
                  if ${pkgs.mysql84}/bin/mysqladmin --protocol=tcp -h127.0.0.1 -P3306 ping >/dev/null 2>&1; then
                    echo "Using existing MySQL on 127.0.0.1:3306"
                    exec sleep 999999999
                  fi
                  ROOT=$(pwd)
                  rm -f "$ROOT/.nix/mysql.sock" "$ROOT/.nix/mysql.sock.lock" "$ROOT/.nix/mysqld.pid" "$ROOT/.nix/mysqld.log"
                  if [ ! -d "$ROOT/.nix/data/mysql/mysql" ]; then
                    ${pkgs.mysql84}/bin/mysqld --initialize-insecure --datadir="$ROOT/.nix/data/mysql"
                  fi
                  exec ${pkgs.mysql84}/bin/mysqld --user=root --datadir="$ROOT/.nix/data/mysql" --socket="$ROOT/.nix/mysql.sock" --pid-file="$ROOT/.nix/mysqld.pid" --log-error="$ROOT/.nix/mysqld.log" --tmpdir="$ROOT/.nix/tmp" --port=3306 --bind-address=127.0.0.1 --disable-plugin=mysqlx
                '';
                readiness_probe.exec.command = "${pkgs.mysql84}/bin/mysqladmin --protocol=tcp -h127.0.0.1 -P3306 ping";
              };
              redis = {
                command = ''
                  if ${pkgs.redis}/bin/redis-cli -h127.0.0.1 -p6379 ping >/dev/null 2>&1; then
                    echo "Using existing Redis on 127.0.0.1:6379"
                    exec sleep 999999999
                  fi
                  exec ${pkgs.redis}/bin/redis-server --port 6379 --bind 127.0.0.1
                '';
                readiness_probe.exec.command = "${pkgs.redis}/bin/redis-cli -h127.0.0.1 -p6379 ping";
              };
              minio = {
                command = ''
                  mkdir -p ./.nix/data/minio
                  export MINIO_ROOT_USER=admin
                  export MINIO_ROOT_PASSWORD=mytheclipse
                  if ${pkgs.minio}/bin/minio admin info --endpoint http://127.0.0.1:9000 >/dev/null 2>&1; then
                    echo "Using existing MinIO on 127.0.0.1:9000"
                    exec sleep 999999999
                  fi
                  exec ${pkgs.minio}/bin/minio server ./.nix/data/minio --address :9000 --console-address :9001
                '';
                readiness_probe.exec.command = "export MINIO_ROOT_USER=admin && export MINIO_ROOT_PASSWORD=mytheclipse && ${pkgs.minio}/bin/minio admin info --endpoint http://127.0.0.1:9000";
              };

              backend-rust = {
                command = "DATABASE_URL=${"$"}{DATABASE_URL:-mysql://root@127.0.0.1:3306/sosmed} JWT_SECRET=${"$"}{JWT_SECRET:-please-change-me-rust-jwt-secret-32-chars} REDIS_URL=${"$"}{REDIS_URL:-redis://127.0.0.1:6379} ${apps-packages.rust-backend}/bin/rustexpress";
                depends_on.mysql.condition = "process_healthy";
                depends_on.redis.condition = "process_started";
              };
              backend-elysia = {
                command = "cd ${apps-packages.elysia}/lib && PORT=${"$"}{PORT:-4092} HOST=0.0.0.0 DATABASE_URL=${"$"}{DATABASE_URL:-mysql://root@127.0.0.1:3306/sosmed} JWT_SECRET=${"$"}{JWT_SECRET:-please-change-me-elysia-jwt-secret-32-chars} REDIS_URL=${"$"}{REDIS_URL:-redis://127.0.0.1:6379} MINIO_ENDPOINT=${"$"}{MINIO_ENDPOINT:-127.0.0.1} MINIO_PORT=${"$"}{MINIO_PORT:-9000} MINIO_USE_SSL=${"$"}{MINIO_USE_SSL:-false} MINIO_ACCESS_KEY=${"$"}{MINIO_ACCESS_KEY:-admin} MINIO_SECRET_KEY=${"$"}{MINIO_SECRET_KEY:-mytheclipse} MINIO_BUCKET_NAME=${"$"}{MINIO_BUCKET_NAME:-api} MINIO_PUBLIC_URL=${"$"}{MINIO_PUBLIC_URL:-http://127.0.0.1:9000} ${pkgs.bun}/bin/bun run dist/index.js";
                depends_on.mysql.condition = "process_healthy";
                depends_on.redis.condition = "process_started";
                depends_on.minio.condition = "process_started";
              };
              
              frontend-nextjs = {
                command = ''
                  export NEXTJS_PORT=${"$"}{NEXTJS_PORT:-3001}
                  if ! ${pkgs.nodejs}/bin/node -e 'const net=require("net"); const p=parseInt(process.env.NEXTJS_PORT,10); const s=net.createServer(); s.once("error",()=>process.exit(1)); s.once("listening",()=>s.close(()=>process.exit(0))); s.listen(p,"127.0.0.1");' >/dev/null 2>&1; then
                    echo "Using existing Next.js on 127.0.0.1:${"$"}{NEXTJS_PORT}"
                    exec sleep 999999999
                  fi
                  cd ${apps-packages.nextjs}/share/nextjs && HOSTNAME=0.0.0.0 PORT=${"$"}{NEXTJS_PORT} ${pkgs.nodejs}/bin/node server.js
                '';
              };
              frontend-solidjs = {
                command = ''
                  export SOLIDJS_PORT=${"$"}{SOLIDJS_PORT:-3010}
                  if ! ${pkgs.nodejs}/bin/node -e 'const net=require("net"); const p=parseInt(process.env.SOLIDJS_PORT,10); const s=net.createServer(); s.once("error",()=>process.exit(1)); s.once("listening",()=>s.close(()=>process.exit(0))); s.listen(p,"127.0.0.1");' >/dev/null 2>&1; then
                    echo "Using existing SolidJS on 127.0.0.1:${"$"}{SOLIDJS_PORT}"
                    exec sleep 999999999
                  fi
                  cd ${apps-packages.solidjs}/share/solidjs && HOST=0.0.0.0 PORT=${"$"}{SOLIDJS_PORT} ${pkgs.bun}/bin/bun .output/server/index.mjs
                '';
              };
            };
          };
        };
    };
}
