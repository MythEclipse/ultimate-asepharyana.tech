"use client";

import React, { useEffect, useState } from 'react';
import AnimeGrid from '@/components/card/AnimeGrid2a';
import Link from 'next/link';
import ButtonA from '@/components/button/ScrollButton';
import Loading from '@/components/misc/loading';

interface Anime {
  title: string;
  slug: string;
  poster: string;
  episode: string;
  anime_url: string;
  rating: string;
  current_episode: string;
  release_day: string;
  newest_release_date: string;
}

interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}

interface completeAnimeData {
  status: string;
  data: Anime[];
  pagination: Pagination;
}

interface DetailAnimePageProps {
  params: Promise<{ slug: string }>;
}

export default function AnimePage({ params }: DetailAnimePageProps) {
  const [data, setData] = useState<completeAnimeData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [resolvedParams, setResolvedParams] = useState<{ slug: string } | null>(null);

  useEffect(() => {
    params.then(setResolvedParams);
  }, [params]);

  useEffect(() => {
    const fetchData = async () => {
      if (!resolvedParams) return;

      try {
        const response = await fetch(
          `/api/anime2/complete-anime/${resolvedParams.slug}`
        );
        
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const result = await response.json();
        
        if (!Array.isArray(result.data)) {
          throw new Error('Invalid data format');
        }
        
        setData(result);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch data');
        console.error('Fetch error:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [resolvedParams]);

  if (loading) {
    return <Loading />;
  }

  if (error) {
    return (
      <main className="p-6">
        <h1 className="text-2xl font-bold mt-8 mb-4">Error Loading Data</h1>
        <p>{error}</p>
      </main>
    );
  }

  if (!data || !Array.isArray(data.data)) {
    return (
      <main className="p-6">
        <h1 className="text-2xl font-bold mt-8 mb-4">No Data Available</h1>
      </main>
    );
  }

  return (
    <main className="p-6">
      <h1 className="dark:text-lighta text-2xl font-bold mt-8 mb-4">
        Complete Anime
      </h1>
      <AnimeGrid animes={data.data} />
      <PaginationComponent pagination={data.pagination} />
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination: Pagination }) => {
  return (
    <div className="flex justify-between mt-8">
      {pagination.has_previous_page && pagination.previous_page !== null && (
        <div className="text-2xl font-bold mt-8 mb-4">
          <Link
            scroll
            href={`/anime2/complete-anime/${pagination.previous_page}`}
            className="text-blue-600 hover:underline"
          >
            <ButtonA>Previous</ButtonA>
          </Link>
        </div>
      )}
      {pagination.has_next_page && pagination.next_page !== null && (
        <div className="text-2xl font-bold mt-8 mb-4">
          <Link
            href={`/anime2/complete-anime/${pagination.next_page}`}
            className="text-blue-600 hover:underline"
          >
            <ButtonA>Next</ButtonA>
          </Link>
        </div>
      )}
    </div>
  );
};