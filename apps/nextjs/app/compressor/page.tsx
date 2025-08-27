'use client';
import { useState } from 'react';
import {
  HiChevronDown,
  HiLink,
  HiCloudUpload,
  HiCheckCircle,
  HiCog,
  HiSparkles,
  HiArrowDown,
  HiXCircle,
  HiCheck,
  HiDownload,
} from 'react-icons/hi';
import stringify from 'safe-json-stringify';

export default function Compressor() {
  const [url, setUrl] = useState('');
  const [size, setSize] = useState('');
  const [uploadMethod, setUploadMethod] = useState<'url' | 'file'>('url');

  interface Result {
    link: string;
    cached: boolean;
    sizeReduction: number; // Added sizeReduction property
  }

  const [result, setResult] = useState<Result | null>(null);
  const [loading, setLoading] = useState(false);
  const [uploadLoading, setUploadLoading] = useState(false);
  const [error, setError] = useState('');

  const handleFileUpload = async (file: File) => {
    try {
      setUploadLoading(true);
      setError('');

      const formData = new FormData();
      formData.append('file', file);

      const uploadRes = await fetch(
        'https://api.ryzendesu.vip/api/uploader/ryzencdn',
        {
          method: 'POST',
          headers: { accept: 'application/json' },
          body: formData,
        }
      );

      const data = await uploadRes.json();
      if (!data.success || !data.url) {
        throw new Error(data.message || 'Upload failed');
      }

      setUrl(data.url);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'File upload failed');
      setUrl('');
    } finally {
      setUploadLoading(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setResult(null);

    if (!url || !size) {
      setError('Please fill all fields');
      return;
    }

    try {
      setLoading(true);
      const res = await fetch(
        `/api/compress?url=${encodeURIComponent(url)}&size=${size}`
      );
      const data = await res.json();

      console.log(stringify(data));
      if (data.status === 'success' && typeof data.data === 'object') {
        setResult(JSON.parse(JSON.stringify(data.data)));
      } else {
        setError('Compression failed');
      }
    } catch {
      setError('Error connecting to server');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className='container mx-auto py-12 px-4 max-w-4xl'>
      {/* Header Section */}
      <div className='text-center mb-16 space-y-3'>
        <h1 className='text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-500 bg-clip-text text-transparent'>
          Media Compressor
        </h1>
        <div className='h-1 w-24 mx-auto bg-gradient-to-r from-blue-400 to-purple-400 rounded-full' />
      </div>

      {/* Main Card */}
      <div className='relative group'>
        <div className='absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-600 rounded-3xl opacity-20 blur-xl group-hover:opacity-30 transition-opacity' />
        <div className='relative bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-3xl shadow-2xl overflow-hidden'>
          <div className='p-8 space-y-8'>
            {/* Upload Method Selector */}
            <div className='space-y-2'>
              <label
                htmlFor='upload-method'
                className='block text-sm font-medium text-gray-600 dark:text-gray-400 mb-2'
              >
                Upload Method
              </label>
              <div className='relative'>
                <select
                  id='upload-method'
                  value={uploadMethod}
                  onChange={(e) =>
                    setUploadMethod(e.target.value as 'url' | 'file')
                  }
                  className='w-full text-blue-500 pl-4 pr-8 py-3.5 bg-gradient-to-r from-blue-50/50 to-purple-50/50 dark:from-gray-800 dark:to-gray-800/70 border-2 border-gray-200 dark:border-gray-700 rounded-xl appearance-none focus:outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-200 dark:focus:ring-blue-900/50 transition-all'
                >
                  <option value='url'>URL Upload</option>
                  <option value='file'>Local File Upload</option>
                </select>
                <HiChevronDown className='w-5 h-5 absolute right-4 top-4 text-gray-500 dark:text-gray-400 pointer-events-none' />
              </div>
            </div>

            {/* Upload Section */}
            <form onSubmit={handleSubmit} className='space-y-8'>
              {uploadMethod === 'url' ? (
                <div className='space-y-2'>
                  <label htmlFor='media-url' className='block text-sm font-medium text-gray-600 dark:text-gray-400'>
                    Media URL
                  </label>
                  <div className='relative'>
                    <input
                      id='media-url'
                      type='url'
                      value={url}
                      onChange={(e) => setUrl(e.target.value)}
                      placeholder='https://example.com/video.mp4'
                      className='w-full pl-12 pr-4 py-3.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-400 focus:ring-2 focus:ring-blue-200 dark:focus:ring-blue-900/50 transition-all shadow-sm'
                      required
                    />
                    <HiLink className='w-5 h-5 absolute left-4 top-4 text-gray-500 dark:text-gray-400' />
                  </div>
                </div>
              ) : (
                <div className='space-y-4'>
                  <label htmlFor='file-upload' className='block text-sm font-medium text-gray-600 dark:text-gray-400'>
                    Upload File
                  </label>
                  <label className='flex flex-col items-center justify-center gap-3 p-8 border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl hover:border-blue-400 cursor-pointer transition-colors bg-gradient-to-b from-white/50 to-blue-50/50 dark:from-gray-800/50 dark:to-gray-900/50'>
                    <div className='p-4 bg-blue-100 dark:bg-gray-800 rounded-full'>
                      <HiCloudUpload className='w-8 h-8 text-blue-500 dark:text-blue-400' />
                    </div>
                    <span className='text-gray-600 dark:text-gray-300 text-center'>
                      {uploadLoading
                        ? 'Uploading...'
                        : 'Drag & drop or click to browse'}
                    </span>
                    <input
                      id='file-upload'
                      type='file'
                      onChange={(e) =>
                        e.target.files?.[0] &&
                        handleFileUpload(e.target.files[0])
                      }
                      className='hidden'
                      accept='video/*,image/*'
                      disabled={uploadLoading}
                    />
                  </label>
                  {url && (
                    <div className='flex items-center gap-2 p-3 bg-green-50 dark:bg-gray-800 rounded-lg'>
                      <HiCheckCircle className='w-5 h-5 text-green-500 flex-shrink-0' />
                      <span className='text-sm text-green-700 dark:text-green-400 truncate'>
                        Ready: {url.split('/').pop()}
                      </span>
                    </div>
                  )}
                </div>
              )}

              {/* Compression Settings */}
              <div className='space-y-2'>
                <label htmlFor='compression-size' className='block text-sm font-medium text-gray-600 dark:text-gray-400'>
                  Compression Settings
                </label>
                <div className='relative'>
                <input
                  id='compression-size'
                  type='text'
                  value={size}
                  onChange={(e) => setSize(e.target.value)}
                  placeholder='Example: 5MB or 50%'
                  className='w-full pl-12 pr-4 py-3.5 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-xl focus:border-blue-400 focus:ring-2 focus:ring-blue-200 dark:focus:ring-blue-900/50 transition-all shadow-sm'
                  required
                />
                <HiCog className='w-5 h-5 absolute left-4 top-4 text-gray-500 dark:text-gray-400' />
                </div>
              </div>

              {/* Submit Button */}
              <button
                type='submit'
                disabled={loading || (uploadMethod === 'file' && !url)}
                className='w-full py-4 px-6 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:scale-[1.02] transition-all disabled:opacity-50 disabled:pointer-events-none relative overflow-hidden'
              >
                {loading && (
                  <div className='absolute inset-0 bg-gradient-to-r from-blue-600/20 to-purple-600/20 animate-pulse' />
                )}
                <span className='relative z-10'>
                  {loading ? (
                    <span className='flex items-center justify-center gap-2'>
                      <HiSparkles className='w-5 h-5 animate-pulse' />
                      Compressing...
                    </span>
                  ) : (
                    <span className='flex items-center justify-center gap-2'>
                      <HiArrowDown className='w-5 h-5' />
                      Start Compression
                    </span>
                  )}
                </span>
              </button>

              {/* Error Message */}
              {error && (
                <div className='flex items-center gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl'>
                  <HiXCircle className='w-6 h-6 text-red-500 flex-shrink-0' />
                  <span className='text-red-700 dark:text-red-400'>
                    {error}
                  </span>
                </div>
              )}
            </form>

            {/* Result Section */}
            {result && (
              <div className='mt-8 p-6 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-gray-800 dark:to-gray-900/50 rounded-2xl border border-blue-200 dark:border-blue-900/50'>
                <div className='text-center space-y-4'>
                  <div className='inline-flex bg-gradient-to-r from-blue-600 to-purple-600 p-4 rounded-2xl shadow-lg'>
                    <HiCheck className='w-8 h-8 text-white' />
                  </div>
                  <h3 className='text-2xl font-bold text-gray-800 dark:text-gray-100'>
                    Compression Successful!
                  </h3>
                  <a
                    href={result.link}
                    target='_blank'
                    rel='noopener noreferrer'
                    className='inline-flex items-center gap-2 px-8 py-3.5 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold rounded-xl shadow-md hover:shadow-lg transition-all'
                  >
                    <HiDownload className='w-5 h-5' />
                    Download File
                  </a>
                  <div className='flex items-center justify-center gap-2 mt-4'>
                    <span className='px-3 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-full text-sm'>
                      {result.cached ? 'Cached Result' : 'New Compression'}
                    </span>
                    <span className='px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 rounded-full text-sm'>
                      Size reduced by {result.sizeReduction}%
                    </span>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
