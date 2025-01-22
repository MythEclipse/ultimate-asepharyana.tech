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
      const data = fs.readFileSync(filePath);

      // Check if the data is JSON and parse it if necessary
      let parsedData;
      try {
        parsedData = JSON.parse(data.toString());
      } catch (err) {
        parsedData = data; // If not JSON, return the raw data
      }

      return { value: parsedData, lastModified: stats.mtimeMs };
    } catch (err) {
      return null;
    }
  }

  async set(key, data, ctx) {
    const filePath = path.join(cacheDir, key);
    
    // Ensure that 'data' is a Buffer or a string before writing to the file system
    let serializedData;
    if (typeof data === 'object') {
      serializedData = Buffer.from(JSON.stringify(data)); // Convert object to JSON string and then to Buffer
    } else if (typeof data === 'string') {
      serializedData = Buffer.from(data); // Handle string data
    } else if (Buffer.isBuffer(data)) {
      serializedData = data; // Handle already serialized data (Buffer)
    } else {
      throw new Error('Data must be a string, Buffer, or an object that can be serialized.');
    }

    await fs.promises.mkdir(path.dirname(filePath), { recursive: true });
    await fs.promises.writeFile(filePath, serializedData);

    // Check total cache size and delete files if it exceeds the max limit
    const files = await readdir(cacheDir);
    let totalSize = 0;
    for (const file of files) {
      const stats = await stat(path.join(cacheDir, file));
      totalSize += stats.size;
    }

    if (totalSize > maxCacheSize) {
      // Sort files by modification time and delete the oldest files first
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
    // Implement tag revalidation logic if needed
  }

  resetRequestCache() {
    // Implement reset cache logic per request if needed
  }
}

module.exports = CacheHandler;
