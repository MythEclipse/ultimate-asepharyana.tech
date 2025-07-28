'use client';

import React, { useState } from 'react';
import Button from '@core/ui/BaseButton';
import { Card } from '@core/ui/ComponentCard';
import { updateUserImage } from '@/lib/prisma/service'; // This will likely change or be removed
import { Loader2 } from 'lucide-react';
import { useAuth } from '@/hooks/AuthContext'; // Import useAuth hook

export default function Settings() {
  const { user } = useAuth(); // Use useAuth hook
  const [image, setImage] = useState<File | null>(null);
  const [status, setStatus] = useState<{
    saving: boolean;
    error: string | null;
  }>({ saving: false, error: null });

  // Redirect if user is not authenticated
  // useEffect(() => {
  //   if (!user && !isLoading) {
  //     // You might want to push to login page or show an access denied message
  //     // For now, we'll assume the route protection will handle this
  //   }
  // }, [user, isLoading]);

  const handleSave = async () => {
    if (!image) {
      setStatus({ saving: false, error: 'Please select an image' });
      return;
    }

    if (!user || !user.id) {
      setStatus({ saving: false, error: 'User not authenticated' });
      return;
    }

    setStatus({ saving: true, error: null });
    try {
      const formData = new FormData();
      formData.append('file', image);

      // Assuming /api/uploader handles authentication internally via JWT
      const response = await fetch('/api/uploader', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Upload failed');
      }

      const { url } = await response.json();

      // This part might need adjustment depending on how user image is stored
      // If it's part of the user object in the JWT, you'd need to re-issue JWT
      // If it's a separate API, ensure it uses JWT for authentication
      // For now, keeping it as is, assuming updateUserImage works with Prisma
      await updateUserImage(user.id, url); // Assuming this function is still valid and handles auth
      setStatus({ saving: false, error: null });
    } catch (err) {
      console.error('Error saving settings:', err);
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
