import React from 'react';

export default function BookmarkLoadingSkeleton() {
  return (
    <main className='min-h-screen p-6 bg-background dark:bg-dark'>
      <div className='max-w-7xl mx-auto space-y-8'>
        {/* Header Skeleton */}
        <div className='flex  gap-6 animate-pulse'>
          <div className='w-14 h-14 bg-zinc-200 dark:bg-zinc-700 rounded-xl' />
          <div className='space-y-2'>
            <div className='h-8 bg-zinc-200 dark:bg-zinc-700 rounded-full w-48' />
            <div className='h-4 bg-zinc-200 dark:bg-zinc-700 rounded-full w-24' />
          </div>
        </div>

        {/* Grid Skeleton - Updated to match AnimeGrid structure */}
        <div className='flex flex-col  p-4'>
          <div className='grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 gap-6 w-full'>
            {[...Array(10)].map((_, i) => (
              <div key={i} className='space-y-3'>
                <div className='bg-zinc-200 dark:bg-zinc-800 aspect-[2/3] rounded-xl animate-pulse' />
                <div className='space-y-2'>
                  <div className='h-4 bg-zinc-200 dark:bg-zinc-700 rounded-full w-4/5' />
                  <div className='h-3 bg-zinc-200 dark:bg-zinc-700 rounded-full w-1/2' />
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Pagination Skeleton */}
        <div className='flex flex-wrap gap-6 justify-between  mt-8 animate-pulse'>
          <div className='flex gap-6'>
            <div className='w-24 h-10 bg-zinc-200 dark:bg-zinc-700 rounded-lg' />
            <div className='w-24 h-10 bg-zinc-200 dark:bg-zinc-700 rounded-lg' />
          </div>
          <div className='w-32 h-4 bg-zinc-200 dark:bg-zinc-700 rounded-full' />
        </div>
      </div>
    </main>
  );
}
