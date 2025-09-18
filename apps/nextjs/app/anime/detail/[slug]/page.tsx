import React from 'react';
import AnimeDetailPageClient from './AnimeDetailPageClient';
import { serverFetch } from '../../../../utils/serverFetch';

export const revalidate = 60;

interface Genre {
  name: string;
  slug: string;
}

interface Episode {
  episode: string;
  slug: string;
}

interface Recommendation {
  slug: string;
  title: string;
  poster: string;
}

interface AnimeData {
  status: string;
  data: {
    title: string;
    alternative_title: string;
    poster: string;
    type: string;
    status: string;
    release_date: string;
    studio: string;
    synopsis: string;
    genres: Genre[];
    producers: string[];
    episode_lists: Episode[];
    recommendations: Recommendation[];
  };
}

async function DetailAnimePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  let initialData: AnimeData | null = null;
  let error: string | null = null;

  try {
    initialData = await serverFetch(`/api/anime/detail/${slug}`);
  } catch (err) {
    console.error('Failed to fetch anime detail data on server:', err);
    error = 'Failed to load anime data';
  }

  return <AnimeDetailPageClient slug={slug} initialData={initialData} initialError={error} />;
}

export default DetailAnimePage;
