# Menambahkan Aplikasi Baru ke Deployment

Dokumentasi ini menjelaskan langkah-langkah menambahkan aplikasi baru ke stack Docker Compose dan deployment VPS pada repositori `ultimate-asepharyana.tech`.

## 1. Struktur Compose

Semua layanan aplikasi ditempatkan di `infra/compose/`.
Setiap layanan memiliki file Compose sendiri seperti:

- `infra/compose/nextjs.yml`
- `infra/compose/rust.yml`
- `infra/compose/picser.yml`

File-file ini digabungkan oleh workflow deployment melalui `docker compose -f ...`.

## 2. Menambahkan layanan baru

Buat file baru `infra/compose/<nama-app>.yml`.
Contoh struktur minimal:

```yaml
services:
  <nama-app>:
    image: ghcr.io/mytheclipse/<nama-app>:<tag>
    restart: always
    networks:
      - app-shared-net
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.<nama-app>.rule=Host(`<subdomain>.asepharyana.tech`)"
      - "traefik.http.routers.<nama-app>.entrypoints=websecure"
      - "traefik.http.routers.<nama-app>.tls=true"
      - "traefik.http.services.<nama-app>.loadbalancer.server.port=<port>"
    env_file:
      - .env.<nama-app>

networks:
  app-shared-net:
    name: app-shared-net
    external: true
```

- Gunakan `app-shared-net` untuk integrasi lintas layanan.
- Pastikan `traefik` sudah tersedia di `infra/compose/traefik.yml`.
- `labels` Traefik mengarahkan `Host()` ke subdomain yang sesuai.

## 3. Environment file khusus

Jika aplikasi membutuhkan variabel lingkungan khusus, buat file env terpisah di `infra/compose/`:

- `infra/compose/.env.<nama-app>`

Contoh untuk `picser`:

```text
GITHUB_TOKEN=ghp_...
GITHUB_OWNER="exceednethunter-code"
GITHUB_REPO="image"
GITHUB_BRANCH="main"
UPSTASH_REDIS_REST_URL="https://glorious-turtle-37188.upstash.io"
UPSTASH_REDIS_REST_TOKEN="..."
REDIS_URL=redis://localhost:6379
```

## 4. Secret GitHub untuk env file

Tambahkan secret Actions yang memuat isi file environment khusus.
Gunakan nama secret uppercase untuk konsistensi, misalnya:

- `ENV_PICSER`

Isi secret adalah seluruh konten file environment multiline.

## 5. Perubahan workflow deployment

Update `.github/workflows/deploy-docker.yml` untuk:

1. Menyertakan secret baru di step deploy.
2. Menulis `.env.<nama-app>` ke `infra/compose/` di VPS.
3. Menambahkan file Compose baru ke daftar `docker compose -f ... pull` dan `up -d`.

Contoh snippet deploy:

```yaml
      - name: Deploy with Docker Compose on VPS
        env:
          ENV_FILE_PRODUCTION: ${{ secrets.ENV_FILE_PRODUCTION }}
          ENV_PICSER: ${{ secrets.ENV_PICSER }}
        run: |
          echo "$ENV_FILE_PRODUCTION" > .env.prod
          echo "$ENV_PICSER" > .env.picser
          scp .env.prod .env.picser "$VPS_USER@$VPS_HOST:$VPS_TARGET_DIR/"
          ssh $SSH_OPTS "$VPS_USER@$VPS_HOST" "mv $VPS_TARGET_DIR/.env.prod $VPS_TARGET_DIR/.env"
```

Untuk file Compose baru, tambahkan `-f infra/compose/<nama-app>.yml` pada kedua perintah `pull` dan `up -d`.

## 6. Contoh `picser`

- File compose: `infra/compose/picser.yml`
- Env file: `infra/compose/.env.picser`
- Secret: `ENV_PICSER`
- Traefik host: `picser.asepharyana.tech`

## 7. Layanan berbagi Traefik

Untuk layanan yang berada di file Compose bersama seperti `infra/compose/shared.yml` dan `infra/compose/monitoring.yml`, tambahkan label Traefik langsung ke service.

Contoh `minio`:

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.minio.rule=Host(`minio.asepharyana.tech`)"
  - "traefik.http.routers.minio.entrypoints=websecure"
  - "traefik.http.routers.minio.tls=true"
  - "traefik.http.services.minio.loadbalancer.server.port=9000"
```

## 8. Trigger workflow

Workflow `deploy-docker.yml` sudah disetel untuk berjalan pada:

- `workflow_run` dari `Build and Push Docker Images`
- `push` ke `infra/compose/**`
- `workflow_dispatch`

Sehingga perubahan Compose langsung memicu deploy.

## 9. Ringkas

Saat menambahkan aplikasi baru:

1. Buat file Compose baru di `infra/compose/`.
2. Tambahkan Traefik label dan `env_file` bila perlu.
3. Buat secret repo untuk env khusus jika ada.
4. Update workflow deploy agar mencakup Compose baru.
5. Pastikan subdomain dan port sudah benar di Traefik labels.
