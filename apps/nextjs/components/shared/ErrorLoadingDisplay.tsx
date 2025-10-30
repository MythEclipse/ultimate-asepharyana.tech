import React from 'react';
import { AlertTriangle, Info } from 'lucide-react';
import { Card, CardContent } from '../ui/card';
import { Skeleton } from '../ui/skeleton';

interface ErrorLoadingDisplayProps {
  type: 'error' | 'loading' | 'no-data';
  message?: string;
  title?: string;
  skeletonType?: 'detail' | 'grid'; // Add skeletonType for different loading states
}

const DetailPageSkeleton = () => (
  <main className="p-4 md:p-8 min-h-screen">
    <div className="max-w-6xl mx-auto">
      <div className="rounded-[24px] p-6 md:p-10 bg-card">
        <div className="flex flex-col md:flex-row items-start gap-8">
          <div className="w-full md:w-1/3 flex flex-col gap-4">
            <Skeleton className="aspect-[2/3] w-full rounded-xl" />
            <Skeleton className="h-12 w-full rounded-full" />
          </div>
          <div className="w-full md:w-2/3 space-y-6">
            <Skeleton className="h-10 w-3/4 rounded-lg" />
            <Card>
              <CardContent className="p-4 grid grid-cols-2 md:grid-cols-4 gap-4">
                {[...Array(4)].map((_, i) => (
                  <div key={i} className="flex items-center gap-3">
                    <Skeleton className="w-10 h-10 rounded-lg" />
                    <div className="space-y-2">
                      <Skeleton className="h-4 w-16" />
                      <Skeleton className="h-4 w-24" />
                    </div>
                  </div>
                ))}
              </CardContent>
            </Card>
            <div className="flex flex-wrap gap-2">
              {[...Array(3)].map((_, i) => (
                <Skeleton key={i} className="h-8 w-24 rounded-full" />
              ))}
            </div>
            <div className="space-y-3">
              <Skeleton className="h-6 w-32" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-5/6" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
);

const GridPageSkeleton = () => (
  <main className="min-h-screen p-6 bg-background dark:bg-dark">
    <div className="max-w-7xl mx-auto space-y-8">
      <div className="flex items-center justify-between mb-6">
        <Skeleton className="h-10 w-1/3" />
      </div>
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
        {[...Array(12)].map((_, i) => (
          <div key={i} className="flex flex-col space-y-2">
            <Skeleton className="aspect-[2/3] w-full rounded-md" />
            <Skeleton className="h-4 w-full" />
            <Skeleton className="h-4 w-3/4" />
          </div>
        ))}
      </div>
    </div>
  </main>
);

export default function ErrorLoadingDisplay({
  type,
  message,
  title,
  skeletonType = 'detail',
}: ErrorLoadingDisplayProps) {
  if (type === 'loading') {
    return skeletonType === 'detail' ? (
      <DetailPageSkeleton />
    ) : (
      <GridPageSkeleton />
    );
  }

  if (type === 'error') {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex items-center gap-4">
            <AlertTriangle className="w-8 h-8 text-red-600 dark:text-red-400" />
            <div>
              <h1 className="text-2xl font-bold text-red-800 dark:text-red-200 mb-2">
                {title || 'Error Loading Data'}
              </h1>
              <p className="text-red-700 dark:text-red-300">
                {message ||
                  'Could not fetch data from the API. Please try again later.'}
              </p>
            </div>
          </div>
        </div>
      </main>
    );
  }

  if (type === 'no-data') {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark">
        <div className="max-w-7xl mx-auto mt-12">
          <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
            <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
            <h1 className="text-2xl font-bold text-blue-800 dark:text-blue-200">
              {title || 'No Data Available'}
            </h1>
            <p className="text-blue-700 dark:text-blue-300">
              {message || 'There is no data to display at this time.'}
            </p>
          </div>
        </div>
      </main>
    );
  }

  return null;
}
