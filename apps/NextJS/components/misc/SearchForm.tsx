'use client';
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Search } from 'lucide-react';

interface SearchFormProps {
  initialQuery: string;
  baseUrl: string;
  classname?: string;
  page?: string;
}

const SearchForm: React.FC<SearchFormProps> = ({
  initialQuery,
  baseUrl,
  classname,
  page,
}) => {
  const [searchQuery, setSearchQuery] = useState(initialQuery);
  const router = useRouter();
  const handleSearch = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (searchQuery.trim()) {
      router.push(
        `${baseUrl}/search/${encodeURIComponent(searchQuery.trim())}/${page || ''}`
      );
    }
  };
  return (
    <div className={classname}>
      <form onSubmit={handleSearch} className='flex items-center gap-4'>
        <div className='relative flex-1'>
          <input
            type='text'
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder='Search anime...'
            className='w-full pl-12 pr-4 py-3 bg-zinc-100 dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 focus:outline-none focus:ring-2 focus:ring-purple-500 dark:focus:ring-purple-400 text-zinc-800 dark:text-zinc-200 placeholder-zinc-500 dark:placeholder-zinc-400 transition-all'
          />
          <Search className='absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-zinc-500 dark:text-zinc-400' />
        </div>
        <button
          type='submit'
          className='px-6 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded-lg font-medium transition-colors flex items-center gap-2'
        >
          Search
          <Search className='w-5 h-5' />
        </button>
      </form>
    </div>
  );
};

export default SearchForm;
