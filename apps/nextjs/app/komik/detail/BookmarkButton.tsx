'use client';

import React from 'react';
import { Button } from '../../../components/ui/button';
import { Bookmark } from 'lucide-react';
import { useBookmark } from '../../../utils/hooks/useBookmark';
import type { KomikBookmark } from '../../../lib/bookmarks';

interface BookmarkButtonProps {
  komikId: string;
  title: string;
  poster: string;
}

export default function BookmarkButton({
  komikId,
  title,
  poster,
}: BookmarkButtonProps) {
  const bookmarkData: KomikBookmark = {
    slug: komikId,
    title,
    poster,
    chapter: '',
    score: '',
    date: '',
    type: '',
    komik_id: komikId,
  };

  const { isBookmarked: bookmarked, toggle: handleBookmark } = useBookmark<KomikBookmark>(
    'komik',
    komikId,
    bookmarkData
  );

  return (
    <Button
      onClick={() => handleBookmark()}
      variant={bookmarked ? 'destructive' : 'default'}
      size="lg"
      className="w-full"
    >
      <Bookmark className="w-5 h-5 mr-2" />
      {bookmarked ? 'Hapus Bookmark' : 'Bookmark'}
    </Button>
  );
}
