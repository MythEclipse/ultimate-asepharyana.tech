'use client';
import React, { useState } from 'react';
import Button from '@core/ui/BaseButton';
import { Card } from '@core/ui/ComponentCard';
import { useSession } from 'next-auth/react';
import { Loader2 } from 'lucide-react';

export default function Settings() {
  const { data: session, status } = useSession();
  const [image, setImage] = useState<File | null>(null);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSave = async () => {
    if (!image) {
      setError('Please select an image');
      return;
    }

    if (status === 'loading' || !session?.user?.id) {
      setError('User not authenticated');
      return;
    }

    setSaving(true);
    setError(null);
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
      
      
      setSaving(false);
    } catch (err) {
      console.error('Error saving settings:', err);
      setError('Failed to update profile');
      setSaving(false);
    }
  };

  return (
    <div className='container mx-auto py-8 px-4 max-w-2xl'>
      <h1 className='text-4xl font-bold text-gray-800 dark:text-gray-100 mb-8 text-center'>
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
            disabled={saving || status === 'loading'}
            className='w-full'
          >
            {saving ? (
              <Loader2 className='w-4 h-4 animate-spin' />
            ) : (
              'Save Changes'
            )}
          </Button>
          
          {error && (
            <div className='text-red-500 text-sm mt-2 text-center'>
              {error}
            </div>
          )}
        </div>
      </Card>
    </div>
  );
}
