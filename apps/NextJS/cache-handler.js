// cache-handler.js
const fs = require('fs');
const path = require('path');
const util = require('util');

const cacheDir = path.resolve(__dirname, '.next/cache');
const maxCacheSize = 5 * 1024 * 1024 * 1024; // 5GB

const stat = util.promisify(fs.stat);
const readdir = util.promisify(fs.readdir);
const unlink = util.promisify(fs.unlink);

class CacheHandler {
  constructor(options) {
    this.options = options;
  }

  async get(key) {
    const filePath = path.join(cacheDir, key);
    try {
      const stats = await stat(filePath);
      return { value: fs.readFileSync(filePath), lastModified: stats.mtimeMs };
    } catch (err) {
      return null;
    }
  }

  async set(key, data, ctx) {
    const filePath = path.join(cacheDir, key);
    await fs.promises.mkdir(path.dirname(filePath), { recursive: true });
    await fs.promises.writeFile(filePath, data);

    // Memeriksa ukuran total cache dan menghapus file jika melebihi batas
    const files = await readdir(cacheDir);
    let totalSize = 0;
    for (const file of files) {
      const stats = await stat(path.join(cacheDir, file));
      totalSize += stats.size;
    }

    if (totalSize > maxCacheSize) {
      // Mengurutkan file berdasarkan waktu modifikasi dan menghapus yang paling lama
      const fileStats = await Promise.all(
        files.map(async (file) => {
          const stats = await stat(path.join(cacheDir, file));
          return { file, mtime: stats.mtimeMs };
        })
      );
      fileStats.sort((a, b) => a.mtime - b.mtime);
      let sizeToFree = totalSize - maxCacheSize;
      for (const { file } of fileStats) {
        const filePath = path.join(cacheDir, file);
        const stats = await stat(filePath);
        if (stats.size <= sizeToFree) {
          await unlink(filePath);
          sizeToFree -= stats.size;
        } else {
          break;
        }
      }
    }
  }

  async revalidateTag(tags) {
    // Implementasi revalidasi tag jika diperlukan
  }

  resetRequestCache() {
    // Implementasi reset cache per permintaan jika diperlukan
  }
}

module.exports = CacheHandler;
