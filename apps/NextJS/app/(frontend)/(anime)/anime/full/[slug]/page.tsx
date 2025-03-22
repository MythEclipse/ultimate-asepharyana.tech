"use client";

import Link from "next/link";
import React, { useEffect, useState } from "react";
import useSWR from "swr";
import { BaseUrl } from "@/lib/url";
import { BackgroundGradient } from "@/components/background/background-gradient";
import ButtonA from "@/components/button/ScrollButton";
import ClientPlayer from "@/components/misc/ClientPlayer";
import Loading from "@/components/misc/loading";

interface AnimeResponse {
  status: string;
  data: AnimeData;
}

interface AnimeData {
  episode: string;
  episode_number: string;
  anime: AnimeInfo;
  has_next_episode: boolean;
  next_episode: EpisodeInfo | null;
  has_previous_episode: boolean;
  previous_episode: EpisodeInfo | null;
  stream_url: string;
  download_urls: Record<string, { server: string; url: string }[]>;
  image_url: string;
}

interface AnimeInfo {
  slug: string;
}

interface EpisodeInfo {
  slug: string;
}

interface DetailAnimePageProps {
  params: Promise<{ slug: string }>;
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function DetailAnimePage({ params }: DetailAnimePageProps) {
  const [resolvedParams, setResolvedParams] = useState<{ slug: string } | null>(null);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    params.then((res) => {
      setResolvedParams(res);
      setMounted(true);
    });
  }, [params]);

  const { data, error, isLoading } = useSWR<AnimeResponse>(
    mounted && resolvedParams ? `${BaseUrl}/api/anime/full/${resolvedParams.slug}` : null,
    fetcher
  );

  if (!mounted || !resolvedParams) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loading />
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4 max-w-screen-md mx-auto">
        <p className="text-red-500">Error loading anime details</p>
      </div>
    );
  }

  if (isLoading || !data) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loading />
      </div>
    );
  }

  if (data.status !== "Ok") {
    return (
      <div className="p-4 max-w-screen-md mx-auto">
        <p className="text-red-500">Error loading anime details</p>
      </div>
    );
  }

  return (
    <BackgroundGradient className="rounded-[22px] p-7 bg-white dark:bg-zinc-900">
      <h1 className="text-4xl font-bold text-white-900">{data.data.episode}</h1>
      <hr className="my-4 border-white-300" />

      <div className="flex flex-col gap-2 mt-4">
        {data.data.stream_url && <ClientPlayer url={data.data.stream_url} />}
        <div className="flex justify-between mt-8">
          {data.data.previous_episode && (
            <p className="text-lg text-white-700">
              <Link scroll href={`/anime/full/${data.data.previous_episode.slug}`}>
                <ButtonA>Previous Episode</ButtonA>
              </Link>
            </p>
          )}
          {data.data.next_episode && (
            <p className="text-lg text-white-700">
              <Link scroll href={`/anime/full/${data.data.next_episode.slug}`}>
                <ButtonA>Next Episode</ButtonA>
              </Link>
            </p>
          )}
        </div>
      </div>

      <hr className="my-4 border-white-300 dark:border-darka" />

      <h2 className="text-3xl font-semibold mt-4 text-white-900">Download Links</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mt-4">
        {Object.entries(data.data.download_urls).map(([resolution, links]) => (
          <BackgroundGradient
            key={resolution}
            className="rounded-[22px] bg-lighta dark:bg-dark p-4 shadow-lg"
          >
            <p className="text-lg font-semibold text-gray-900 dark:text-gray-100">
              {resolution}
            </p>
            <div className="flex flex-col gap-3 mt-3">
              {links.map((link, index) => (
                <div key={index}>
                  <p className="text-md text-gray-800 dark:text-gray-300">{link.server}</p>
                  <ButtonA className="text-center" href={link.url}>
                    <div className="text-lg font-bold">Download</div>
                  </ButtonA>
                </div>
              ))}
            </div>
          </BackgroundGradient>
        ))}
      </div>
    </BackgroundGradient>
  );
}
