// cache-handler.js
const fs = require('fs');
const path = require('path');
const util = require('util');
const stream = require('stream');

const cacheDir = path.resolve(__dirname, '.next/cache');
const maxCacheSize = 5 * 1024 * 1024 * 1024; // 5GB

const pipeline = util.promisify(stream.pipeline);
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
      // Return a readable stream instead of loading entire file
      const readable = fs.createReadStream(filePath);
      return {
        value: readable,
        lastModified: (await stat(filePath)).mtimeMs
      };
    } catch (err) {
      return null;
    }
  }

  async set(key, data, ctx) {
    const filePath = path.join(cacheDir, key);
    await fs.promises.mkdir(path.dirname(filePath), { recursive: true });

    // Handle different data types properly
    if (data instanceof stream.Readable) {
      // Pipe stream directly to file
      const writable = fs.createWriteStream(filePath);
      await pipeline(data, writable);
    } else {
      // Handle buffers and JSON
      const content = typeof data === 'object' 
        ? JSON.stringify(data)
        : data;
      await fs.promises.writeFile(filePath, content);
    }

    // Improved cache cleanup with LRU strategy
    const files = await readdir(cacheDir);
    const fileStats = await Promise.all(
      files.map(async (file) => ({
        file,
        stats: await stat(path.join(cacheDir, file))
      }))
    );

    // Sort by last modified time
    fileStats.sort((a, b) => a.stats.mtimeMs - b.stats.mtimeMs);

    // Calculate total size
    let totalSize = fileStats.reduce((acc, { stats }) => acc + stats.size, 0);

    // Delete oldest files until under limit
    while (totalSize > maxCacheSize && fileStats.length > 0) {
      const oldest = fileStats.shift();
      await unlink(path.join(cacheDir, oldest.file));
      totalSize -= oldest.stats.size;
    }
  }

  async revalidateTag(tags) {
    // Implementation if needed
  }

  resetRequestCache() {
    // Implementation if needed
  }
}

module.exports = CacheHandler;