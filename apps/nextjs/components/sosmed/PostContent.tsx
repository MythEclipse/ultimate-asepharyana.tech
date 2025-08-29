import React from 'react';
import Image from 'next/image';
import { Textarea } from '../text/textarea';
import ButtonA from '../ui/BaseButton';
import { Loader2 } from 'lucide-react';

interface PostContentProps {
  content: string;
  imageUrl?: string | null;
  isEditing: boolean;
  editedContent: string;
  onContentChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onSave: () => void;
  onCancelEdit: () => void;
  isSaving: boolean;
}

export default function PostContent({
  content,
  imageUrl,
  isEditing,
  editedContent,
  onContentChange,
  onSave,
  onCancelEdit,
  isSaving,
}: PostContentProps) {
  return (
    <>
      {isEditing ? (
        <div className="mb-6 space-y-4">
          <Textarea
            value={editedContent}
            onChange={onContentChange}
            className="min-h-[120px] text-lg border-2 border-blue-200 focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl transition-all"
            aria-label="Edit post content"
          />
          <div className="flex gap-3">
            <ButtonA
              onClick={onSave}
              className="bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 text-white shadow-lg hover:shadow-xl transition-all"
              disabled={isSaving}
              aria-label="Save changes"
            >
              {isSaving ? (
                <div className="flex items-center justify-center gap-2">
                  <Loader2
                    className="w-5 h-5 animate-spin"
                    aria-hidden="true"
                  />
                  <span>Saving...</span>
                </div>
              ) : (
                'Save Changes'
              )}
            </ButtonA>
            <ButtonA
              onClick={onCancelEdit}
              className="bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
              aria-label="Cancel editing"
            >
              Cancel
            </ButtonA>
          </div>
        </div>
      ) : (
        <p className="text-gray-800 dark:text-gray-200 mb-6 text-lg leading-relaxed">
          {content}
        </p>
      )}

      {imageUrl && (
        <div className="relative mb-6 rounded-xl overflow-hidden border border-gray-200 dark:border-gray-800">
          <Image
            src={imageUrl}
            alt="Post image"
            width={800}
            unoptimized
            height={450}
            className="object-cover w-full hover:scale-105 transition-transform duration-300 cursor-pointer"
          />
        </div>
      )}
    </>
  );
}
