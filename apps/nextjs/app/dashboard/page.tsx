'use client';

import { useEffect, useState } from 'react';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '../../components/ui/ComponentCard';
import { getBookmarks } from '../../lib/bookmarks';
import ProtectedRoute from '../../components/auth/ProtectedRoute';

export default function DashboardPage() {
  const [animeCount, setAnimeCount] = useState(0);
  const [komikCount, setKomikCount] = useState(0);

  useEffect(() => {
    setAnimeCount(getBookmarks('anime').length);
    setKomikCount(getBookmarks('komik').length);
  }, []);

  return (
    <ProtectedRoute>
      <main className="p-6 grid grid-cols-1 sm:grid-cols-2 gap-6 max-w-4xl mx-auto">
      <Card className="shadow-lg rounded-2xl w-full">
        <CardHeader>
          <CardTitle className="text-2xl font-bold">
            Total Anime Bookmarked
          </CardTitle>
        </CardHeader>
        <CardContent className="text-4xl font-bold text-center">
          {animeCount}
        </CardContent>
      </Card>
      <Card className="shadow-lg rounded-2xl w-full">
        <CardHeader>
          <CardTitle className="text-2xl font-bold">
            Total Komik Bookmarked
          </CardTitle>
        </CardHeader>
        <CardContent className="text-4xl font-bold text-center">
          {komikCount}
        </CardContent>
      </Card>
    </main>
    </ProtectedRoute>
  );
}
