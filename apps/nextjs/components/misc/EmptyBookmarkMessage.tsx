import React from 'react';
import { Info } from 'lucide-react';

export default function EmptyBookmarkMessage() {
  return (
    <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
      <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
      <div>
        <h2 className="text-xl font-medium text-blue-800 dark:text-blue-200 mb-2">
          No Bookmarked Anime
        </h2>
        <p className="text-blue-700 dark:text-blue-300">
          Start bookmarking your favorite anime to see them here!
        </p>
      </div>
    </div>
  );
}
