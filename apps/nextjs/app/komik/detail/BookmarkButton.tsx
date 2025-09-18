'use client';

import React, { useState, useEffect } from 'react';
import { Button } from '../../../components/ui/button';
import { Bookmark } from 'lucide-react';

interface BookmarkButtonProps {
  komikId: string;
  title: string;
  poster: string;
}

export default function BookmarkButton({ komikId, title, poster }: BookmarkButtonProps) {
  const [bookmarked, setBookmarked] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined' && komikId) {
      const bookmarks = JSON.parse(
        localStorage.getItem('bookmarks-komik') || '[]',
      );
      setBookmarked(
        bookmarks.some((item: { slug: string }) => item.slug === komikId),
      );
    }
  }, [komikId]);

  const handleBookmark = () => {
    let bookmarks = JSON.parse(localStorage.getItem('bookmarks-komik') || '[]');
    const isBookmarked = bookmarks.some(
      (item: { slug: string }) => item.slug === komikId,
    );

    if (isBookmarked) {
      bookmarks = bookmarks.filter(
        (item: { slug: string }) => item.slug !== komikId,
      );
    } else {
      bookmarks.push({
        slug: komikId,
        title,
        poster,
      });
    }
    localStorage.setItem('bookmarks-komik', JSON.stringify(bookmarks));
    setBookmarked(!isBookmarked);
  };

  return (
    <Button
      onClick={handleBookmark}
      variant={bookmarked ? 'destructive' : 'default'}
      size="lg"
      className="w-full"
    >
      <Bookmark className="w-5 h-5 mr-2" />
      {bookmarked ? 'Hapus Bookmark' : 'Bookmark'}
    </Button>
  );
}
