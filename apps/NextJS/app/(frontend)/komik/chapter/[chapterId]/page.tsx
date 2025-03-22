"use client";

import React, { useEffect, useState } from 'react';
import useSWR from 'swr';
import Link from 'next/link';
import Image from 'next/image';
import { BaseUrl } from '@/lib/url';
import ButtonA from '@/components/button/ScrollButton';
import Loading from '@/components/misc/loading';

interface ChapterDetail {
  title: string;
  komik_id: string;
  prev_chapter_id: string;
  next_chapter_id: string;
  list_chapter: string;
  downloadUrl: string;
  images: string[];
}

interface PageProps {
  params: Promise<{ chapterId: string }>;
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function ChapterPage({ params }: PageProps) {
  const [resolvedParams, setResolvedParams] = useState<{ chapterId: string } | null>(null);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    params.then(setResolvedParams);
    setMounted(true);
  }, [params]);

  const { data: chapter, error, isLoading } = useSWR<ChapterDetail>(
    mounted && resolvedParams 
      ? `${BaseUrl}/api/komik/chapter?chapter_url=${resolvedParams.chapterId}`
      : null,
    fetcher
  );

  if (!mounted || !resolvedParams) {
    return <div className="min-h-screen flex items-center justify-center"><Loading /></div>;
  }

  if (error) {
    return (
      <main className='p-6 pb-20'>
        <div className='text-center text-red-500'>
          <h1 className='text-2xl font-bold'>Error Loading Chapter</h1>
          <p>Failed to load chapter data. Please try again later.</p>
        </div>
      </main>
    );
  }

  if (isLoading || !chapter) {
    return <div className="min-h-screen flex items-center justify-center"><Loading /></div>;
  }

  return (
    <main className='p-6 pb-20'>
      <div className='text-center mb-4'>
        <h1 className='text-2xl font-bold dark:text-white'>{chapter.title}</h1>
        <div className='mt-4 grid grid-cols-3 gap-4'>
          <div className='flex justify-start'>
            {chapter.prev_chapter_id && (
              <Link scroll href={`/komik/chapter/${chapter.prev_chapter_id}`}>
                <ButtonA>Previous Chapter</ButtonA>
              </Link>
            )}
          </div>
          <div className='flex justify-center'>
            <Link href={`/komik/detail/${chapter.list_chapter}`}>
              <ButtonA>Back to List Chapter</ButtonA>
            </Link>
          </div>
          <div className='flex justify-end'>
            {chapter.next_chapter_id && (
              <Link scroll href={`/komik/chapter/${chapter.next_chapter_id}`}>
                <ButtonA>Next Chapter</ButtonA>
              </Link>
            )}
          </div>
        </div>
      </div>

      <div className='flex flex-col md:w-1/2 md:mx-auto'>
        {chapter.images.map((image, index) => (
          <div
            key={index}
            style={{
              position: 'relative',
              width: '100%',
              minHeight: '300px',
              backgroundColor: '#f0f0f0',
            }}
          >
            <Image
              src={`${BaseUrl}/api/imageproxy?url=${encodeURIComponent(image)}`}
              alt={`Chapter ${chapter.title} - page ${index + 1}`}
              className='object-cover transition-opacity duration-300'
              width={725}
              height={1024}
              unoptimized
            />
          </div>
        ))}
      </div>

      <div className='mt-4 grid grid-cols-3 gap-4'>
        {/* Duplicate navigation for bottom */}
        <div className='flex justify-start'>
          {chapter.prev_chapter_id && (
            <Link scroll href={`/komik/chapter/${chapter.prev_chapter_id}`}>
              <ButtonA>Previous Chapter</ButtonA>
            </Link>
          )}
        </div>
        <div className='flex justify-center'>
          <Link href={`/komik/detail/${chapter.list_chapter}`}>
            <ButtonA>Back to List Chapter</ButtonA>
          </Link>
        </div>
        <div className='flex justify-end'>
          {chapter.next_chapter_id && (
            <Link scroll href={`/komik/chapter/${chapter.next_chapter_id}`}>
              <ButtonA>Next Chapter</ButtonA>
            </Link>
          )}
        </div>
      </div>
    </main>
  );
}