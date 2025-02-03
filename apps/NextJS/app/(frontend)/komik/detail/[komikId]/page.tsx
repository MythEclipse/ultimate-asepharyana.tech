"use client";

import React, { useState, useEffect } from "react";
import useSWR from "swr";
import { BaseUrl } from "@/lib/url";
import Image from "next/image";
import Link from "next/link";
import { BackgroundGradient } from "@/components/background/background-gradient";
import ButtonA from "@/components/button/ScrollButton";
import Loading from "./loading";

interface Genre {
  name: string;
  slug: string;
}

interface Chapter {
  chapter: string;
  date: string;
  chapter_id: string;
}

interface Recommendation {
  slug: string;
  title: string;
  image: string;
}

interface MangaData {
  title: string;
  alternativeTitle: string;
  image: string;
  score: string;
  description: string;
  status: string;
  type: string;
  releaseDate: string;
  author: string;
  artist: string;
  serialization: string;
  postedBy: string;
  postedOn: string;
  updatedOn: string;
  genres: Genre[];
  chapters: Chapter[];
  recommendations: Recommendation[];
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailMangaPage({ params }: { params: { komikId: string } }) {
  const { data: manga, error } = useSWR<MangaData>(
    `${BaseUrl}/api/komik/detail?komik_id=${params.komikId}`,
    fetcher
  );

  const [bookmarked, setBookmarked] = useState(false);

  useEffect(() => {
    if (typeof window !== "undefined" && manga) {
      const bookmarks = JSON.parse(localStorage.getItem("bookmarks-komik") || "[]");
      setBookmarked(bookmarks.some((item: { slug: string }) => item.slug === params.komikId));
    }
  }, [manga, params.komikId]);

  const handleBookmark = () => {
    if (!manga) return;

    let bookmarks = JSON.parse(localStorage.getItem("bookmarks-komik") || "[]");

    if (bookmarked) {
      bookmarks = bookmarks.filter((item: { slug: string }) => item.slug !== params.komikId);
    } else {
      bookmarks.push({
        slug: params.komikId,
        title: manga.title,
        poster: manga.image,
      });
    }

    localStorage.setItem("bookmarks-komik", JSON.stringify(bookmarks));
    setBookmarked(!bookmarked);
  };

  if (error) return <p className="text-red-500 text-center">Failed to load manga data</p>;
  if (!manga) return <Loading />;

  return (
    <main className='p-6 bg-background dark:bg-dark min-h-screen'>
      <div className='max-w-4xl mx-auto bg-white dark:bg-dark rounded-lg shadow-lg'>
        <BackgroundGradient className='rounded-[22px] p-7 bg-white dark:bg-zinc-900'>
          <div className='flex flex-col md:flex-row items-center md:items-start'>
            <div className='w-full md:w-1/3 mb-6 md:mb-0 flex justify-center md:justify-start'>
              <Image
                src={manga.image}
                alt={manga.title}
                width={330}
                height={450}
                className='object-cover rounded-lg shadow-md'
                priority
              />
            </div>
            <div className='w-full md:w-2/3 md:pl-6'>
              <h1 className='text-3xl font-bold mb-4 text-primary-dark dark:text-primary'>
                {manga.title}
              </h1>
              <button
                onClick={handleBookmark}
                className={`px-4 py-2 rounded text-white ${
                  bookmarked ? "bg-red-500" : "bg-blue-500"
                } mb-4`}
              >
                {bookmarked ? "Unbookmark" : "Bookmark"}
              </button>
              <div className='text-gray-800 dark:text-gray-200 mb-4'>
                <p className='mb-2'>
                  <strong>Alternative Title:</strong> {manga.alternativeTitle}
                </p>
                <p className='mb-2'>
                  <strong>Score:</strong> {manga.score}
                </p>
                <p className='mb-2'>
                  <strong>Status:</strong> {manga.status}
                </p>
                <p className='mb-2'>
                  <strong>Author:</strong> {manga.author}
                </p>
                <p className='mb-2'>
                  <strong>Type:</strong> {manga.type}
                </p>
                <p className='mb-2'>
                  <strong>Release Date:</strong> {manga.releaseDate}
                </p>
                <p className='mb-4'>
                  <strong>Genres:</strong>{" "}
                  {manga.genres.map((genre) => genre.name).join(", ")}
                </p>
                <p className='mb-4'>
                  <strong>Description:</strong> {manga.description}
                </p>
              </div>

              <div className='mt-6'>
                <h2 className='text-2xl font-semibold mb-2 text-primary-dark dark:text-primary'>
                  Chapters
                </h2>
                <div className='grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-3 gap-4'>
                  {manga.chapters?.length > 0 ? (
                    manga.chapters.map((chapter) => (
                      <Link
                        scroll
                        key={chapter.chapter_id}
                        href={`/komik/chapter/${chapter.chapter_id}`}
                        className=''
                      >
                        <ButtonA className='w-full text-center flex flex-col items-center justify-center'>
                          <span className='text-lg font-bold mb-1 text-center truncate text-primary-dark dark:text-primary'>
                            {chapter.chapter}
                          </span>
                          <span className='text-sm text-center truncate text-gray-500 dark:text-gray-400'>
                            {chapter.date}
                          </span>
                        </ButtonA>
                      </Link>
                    ))
                  ) : (
                    <p className='col-span-full text-center text-primary-dark dark:text-primary'>
                      No chapters available
                    </p>
                  )}
                </div>
              </div>

              <div className='mt-6'>
                <h2 className='text-2xl font-semibold mb-2 text-primary-dark dark:text-primary'>
                  Recommendations
                </h2>
                <div className='overflow-x-auto'>
                  <div className='flex space-x-4'>
                    {manga.recommendations?.length > 0 ? (
                      manga.recommendations.map((recommendation) => (
                        <Link
                          key={recommendation.slug}
                          href={`/komik/detail/${recommendation.slug}`}
                          className='flex-shrink-0 w-64'
                        >
                          <div className='relative h-80 rounded-lg overflow-hidden'>
                            <Image
                              src={recommendation.image}
                              alt={recommendation.title}
                              fill
                              className='object-cover'
                            />
                          </div>
                          <h3 className='mt-2 text-center font-medium text-primary-dark dark:text-primary'>
                            {recommendation.title}
                          </h3>
                        </Link>
                      ))
                    ) : (
                      <p className='col-span-full text-center text-primary-dark dark:text-primary'>
                        No recommendations available
                      </p>
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </BackgroundGradient>
      </div>
    </main>
  );
}