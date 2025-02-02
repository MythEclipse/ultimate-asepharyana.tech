'use client'
import { useState } from 'react'

export default function Compressor() {
  const [url, setUrl] = useState('')
  const [size, setSize] = useState('')
  const [uploadMethod, setUploadMethod] = useState('url')

  interface Result {
    link: string
    cached: boolean
  }

  const [result, setResult] = useState<Result | null>(null)
  const [loading, setLoading] = useState(false)
  const [uploadLoading, setUploadLoading] = useState(false)
  const [error, setError] = useState('')

  const handleFileUpload = async (file: File) => {
    try {
      setUploadLoading(true)
      setError('')

      const formData = new FormData()
      formData.append('file', file)

      const uploadRes = await fetch(
        'https://api.ryzendesu.vip/api/uploader/ryzencdn',
        {
          method: 'POST',
          headers: {
            accept: 'application/json'
          },
          body: formData
        }
      )

      const data = await uploadRes.json()

      if (!data.success || !data.url) {
        throw new Error(data.message || 'Upload failed')
      }

      setUrl(data.url)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'File upload failed')
      setUrl('')
    } finally {
      setUploadLoading(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')
    setResult(null)

    if (!url || !size) {
      setError('Please fill all fields')
      return
    }

    try {
      setLoading(true)
      const res = await fetch(
        `/api/compress?url=${encodeURIComponent(url)}&size=${size}`
      )
      const data = await res.json()

      if (data.status === 'success') {
        setResult(data.data)
      } else {
        setError('Compression failed')
      }
    } catch (err) {
      setError('Error connecting to server')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className='container mx-auto py-8 px-4'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Media Compressor
      </h1>

      <div className='max-w-2xl mx-auto'>
        <div className='border border-blue-500 ring ring-blue-500 shadow-lg rounded-xl p-6 bg-white dark:bg-black'>
          <div className='mb-6'>
            <label className='block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2'>
              Upload Method
            </label>
            <select
              value={uploadMethod}
              onChange={(e) => setUploadMethod(e.target.value)}
              className='w-full p-3 border-2 border-blue-500/30 rounded-lg bg-white dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all'
            >
              <option value='url'>URL</option>
              <option value='file'>File Upload</option>
            </select>
          </div>

          <form onSubmit={handleSubmit} className='space-y-6'>
            {uploadMethod === 'url' ? (
              <div>
                <label className='block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2'>
                  Media URL
                </label>
                <input
                  type='url'
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  placeholder='https://example.com/video.mp4'
                  className='w-full p-3 border-2 border-blue-500/30 rounded-lg dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent shadow-sm'
                  required
                />
              </div>
            ) : (
              <div>
                <label className='block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2'>
                  Upload File
                </label>
                <input
                  type='file'
                  onChange={(e) =>
                    e.target.files?.[0] && handleFileUpload(e.target.files[0])
                  }
                  className='w-full text-sm text-gray-500 dark:text-gray-300 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 dark:file:bg-gray-700 file:text-blue-700 dark:file:text-gray-300 hover:file:bg-blue-100 dark:hover:file:bg-gray-600'
                  accept='video/*,image/*'
                  disabled={uploadLoading}
                />
                {uploadLoading && (
                  <p className='mt-2 text-sm text-blue-600'>
                    Uploading file...
                  </p>
                )}
                {url && (
                  <p className='mt-2 text-sm text-blue-600 truncate'>
                    Uploaded: {url}
                  </p>
                )}
              </div>
            )}

            <div>
              <label className='block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2'>
                Compression Size/Quality
              </label>
              <input
                type='text'
                value={size}
                onChange={(e) => setSize(e.target.value)}
                placeholder='5 or 50%'
                className='w-full p-3 border-2 border-blue-500/30 rounded-lg dark:bg-gray-700 focus:ring-2 focus:ring-blue-500 focus:border-transparent shadow-sm'
                required
              />
            </div>

            <button
              type='submit'
              disabled={loading || (uploadMethod === 'file' && !url)}
              className={`w-full py-3 px-6 ${
                loading ? 'bg-blue-400' : 'bg-blue-600'
              } text-white rounded-lg font-semibold hover:bg-blue-700 transition-colors duration-300 disabled:opacity-50 disabled:cursor-not-allowed`}
            >
              {loading ? 'Processing...' : 'Compress Media'}
            </button>

            {error && (
              <div className='mt-4 p-3 bg-red-100 text-red-700 rounded-lg'>
                {error}
              </div>
            )}
          </form>

          {result && (
            <div className='mt-8 p-4 bg-blue-50 dark:bg-gray-700 rounded-lg text-center'>
              <p className='text-lg font-semibold text-blue-600 dark:text-blue-400 mb-4'>
                🎉 Compression successful!
              </p>
              <a
                href={result.link}
                target='_blank'
                rel='noopener noreferrer'
                className='inline-flex items-center px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors duration-300'
              >
                <span className='mr-2'>⬇️</span>
                Download Compressed File
              </a>
              <p className='mt-3 text-sm text-blue-500 dark:text-blue-300'>
                {result.cached ? '(Cached version)' : '(Newly processed)'}
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
