'use client';

import React, { useState } from 'react';
import { useSession } from 'next-auth/react';
import Button from '@/components/button/NormalButton';
import { Card } from '@/components/card/ComponentCard';
import { updateUserImage } from '@/lib/prisma/service';
import { Loader2 } from 'lucide-react';

export default function Settings() {
  const { data: session } = useSession();
  const [image, setImage] = useState<File | null>(null);
  const [status, setStatus] = useState<{
    saving: boolean;
    error: string | null;
  }>({ saving: false, error: null });

  const handleSave = async () => {
    if (!image) {
      setStatus({ saving: false, error: 'Please select an image' });
      return;
    }

    setStatus({ saving: true, error: null });
    try {
      const formData = new FormData();
      formData.append('file', image);

      const response = await fetch('/api/uploader', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Upload failed');
      }

      const { url } = await response.json();

      if (!session?.user?.id) {
        throw new Error('User ID is undefined');
      }

      await updateUserImage(session.user.id, url);
      setStatus({ saving: false, error: null });
    } catch {
      setStatus({ saving: false, error: 'Failed to update profile' });
    }
  };

  return (
    <div className='container mx-auto py-8 px-4 max-w-2xl'>
      <h1 className='text-4xl font-extrabold text-gray-800 dark:text-gray-100 mb-8 text-center'>
        Edit Profile
      </h1>

      <Card>
        <div className='p-4 space-y-4'>
          <div>
            <label className='block text-sm font-medium text-gray-700 dark:text-gray-300'>
              Profile Image
            </label>
            <input
              type='file'
              accept='image/*'
              onChange={(e) => setImage(e.target.files?.[0] || null)}
              className='mt-1 block w-full'
            />
          </div>

          <Button
            onClick={handleSave}
            disabled={status.saving}
            className='w-full'
          >
            {status.saving ? (
              <Loader2 className='w-4 h-4 animate-spin' />
            ) : (
              'Save Changes'
            )}
          </Button>

          {status.error && (
            <div className='text-red-500 text-sm mt-2 text-center'>
              {status.error}
            </div>
          )}
        </div>
      </Card>
    </div>
  );
}
