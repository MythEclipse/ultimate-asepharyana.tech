import React from 'react';
import Image from 'next/image';
import { HiBadgeCheck, HiPencil, HiTrash } from 'react-icons/hi';
import { formatDistanceToNow } from 'date-fns';

interface PostHeaderProps {
  postId: string;
  userName: string | null;
  userImage: string | null;
  createdAt: Date;
  authenticatedUserId: string | undefined;
  postUserId: string;
  onEditPost: () => void;
  onDeletePost: () => void;
  isEditingPost: boolean;
  isDeletingPost: boolean;
}

export default function PostHeader({
  postId,
  userName,
  userImage,
  createdAt,
  authenticatedUserId,
  postUserId,
  onEditPost,
  onDeletePost,
  isEditingPost,
  isDeletingPost,
}: PostHeaderProps) {
  return (
    <div className="flex items-start gap-4 mb-6">
      <div className="relative">
        <Image
          src={userImage || '/default-profile.png'}
          alt={userName || 'User profile picture'}
          width={56}
          height={56}
          className="rounded-full border-2 border-blue-400/80 shadow-md hover:border-blue-500 transition-all cursor-pointer"
        />
        <div className="absolute -bottom-1 -right-1 bg-blue-500 text-white rounded-full px-2 py-0.5 text-xs shadow-sm">
          <HiBadgeCheck className="w-4 h-4" aria-hidden="true" />
        </div>
      </div>

      <div className="flex-1">
        <div className="flex items-center gap-3">
          <h2
            id={`post-author-${postId}`}
            className="text-lg font-bold text-gray-800 dark:text-gray-100 hover:text-blue-600 cursor-pointer"
          >
            {userName}
          </h2>
          <span className="text-sm text-gray-500 dark:text-gray-400">
            · {formatDistanceToNow(new Date(createdAt), { addSuffix: true })}
          </span>
        </div>
        <p className="text-sm text-gray-500 dark:text-gray-400">
          @{userName?.replace(/\s+/g, '').toLowerCase()}
        </p>
      </div>

      {authenticatedUserId === postUserId && (
        <div className="flex space-x-3 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            onClick={onEditPost}
            className="p-2 hover:bg-blue-50 dark:hover:bg-gray-800 rounded-full text-blue-500 hover:text-blue-600 transition-colors"
            title="Edit post"
            aria-label="Edit post"
            disabled={isEditingPost}
          >
            <HiPencil className="w-5 h-5" aria-hidden="true" />
          </button>
          <button
            onClick={onDeletePost}
            className="p-2 hover:bg-red-50 dark:hover:bg-gray-800 rounded-full text-red-500 hover:text-red-600 transition-colors"
            title="Delete post"
            aria-label="Delete post"
            disabled={isDeletingPost}
          >
            <HiTrash className="w-5 h-5" aria-hidden="true" />
          </button>
        </div>
      )}
    </div>
  );
}
