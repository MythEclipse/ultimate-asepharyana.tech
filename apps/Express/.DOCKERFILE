# Gunakan image Node.js versi yang sesuai dan berbasis Alpine untuk ukuran yang lebih kecil
FROM node:22-alpine AS builder

# Install pnpm
RUN npm install -g pnpm

# Set working directory di dalam container
WORKDIR /app

# Salin hanya file yang diperlukan untuk installasi dependencies
COPY package.json ./

# Install dependencies
RUN pnpm install 

# Salin semua file setelah dependencies terinstall
COPY . .

# Build aplikasi (jika ada, misalnya TypeScript)
# RUN pnpm run build  # Jika Anda menggunakan TypeScript atau build step lain

# Gunakan image Node.js yang lebih ringan untuk production
FROM node:22-alpine AS production

# Install pnpm
RUN npm install -g pnpm

# Set working directory di dalam container
WORKDIR /app

# Salin file build dari tahap builder
COPY --from=builder /app /app

# Install dependencies untuk production saja
RUN pnpm install --prod 

# Tentukan port yang akan digunakan aplikasi
EXPOSE 4090

# Jalankan aplikasi Express.js
CMD ["node", "index.js"]
