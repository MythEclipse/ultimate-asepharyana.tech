import React from 'react';
import { notFound } from 'next/navigation';
import Link from 'next/link';
import Image from 'next/image';
import { BaseUrl } from '@/lib/url';
import ButtonA from '@/components/button/ScrollButton';

interface ChapterDetail {
  title: string;
  komik_id: string;
  prev_chapter_id: string;
  next_chapter_id: string;
  list_chapter: string;
  downloadUrl: string;
  images: string[];
}

export default async function ChapterPage(props: {
  params: Promise<{ chapterId: string }>;
}) {
  const params = await props.params;
  const { chapterId } = params;
  const BaseUrl2 = 'https://jadwiodahwduodh-vee9.vercel.app';
  const response = await fetch(
    `${BaseUrl}/api/komik/chapter?chapter_url=${chapterId}`
  );
  if (!response.ok) {
    notFound();
  }

  const chapter: ChapterDetail = await response.json();

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
              minHeight: '300px', // Placeholder height, adjust as needed
              backgroundColor: '#f0f0f0', // Placeholder background color
            }}
            className=''
          >
            <Image
              src={`${BaseUrl2}/api/imageproxy?url=${encodeURIComponent(image)}`}
              alt={`Chapter ${chapter.title} - page ${index + 1}`}
              className='object-cover transition-opacity duration-300'
              width='725'
              height='1024'
              unoptimized
            />
          </div>
        ))}
      </div>

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
    </main>
  );
}
