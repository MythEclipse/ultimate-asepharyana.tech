import React from 'react';
import { HiHeart, HiChatAlt } from 'react-icons/hi';
import { Loader2 } from 'lucide-react';

interface PostActionsProps {
  postId: string;
  likesCount: number;
  commentsCount: number;
  userHasLiked: boolean;
  onLike: (postId: string) => void;
  onUnlike: (postId: string) => void;
  onToggleComments: (postId: string, show: boolean) => void;
  isLiking: boolean;
  isCommenting: boolean;
  showComments: boolean;
}

export default function PostActions({
  postId,
  likesCount,
  commentsCount,
  userHasLiked,
  onLike,
  onUnlike,
  onToggleComments,
  isLiking,
  isCommenting,
  showComments,
}: PostActionsProps) {
  return (
    <div className="flex items-center gap-6 border-t border-gray-100 dark:border-gray-800 pt-6">
      <button
        onClick={() => (userHasLiked ? onUnlike(postId) : onLike(postId))}
        className="flex items-center gap-2 group/like"
        disabled={isLiking}
        aria-label={userHasLiked ? 'Unlike post' : 'Like post'}
        aria-pressed={userHasLiked}
      >
        <div className="p-2 rounded-full bg-gray-100 dark:bg-gray-800 group-hover/like:bg-red-50 dark:group-hover/like:bg-red-900/20 transition-colors">
          <HiHeart
            className={`w-6 h-6 ${
              userHasLiked
                ? 'text-red-500 fill-current'
                : 'text-gray-500 dark:text-gray-400'
            } group-hover/like:text-red-500 transition-colors`}
            aria-hidden="true"
          />
        </div>
        {isLiking ? (
          <Loader2 className="w-5 h-5 animate-spin" aria-hidden="true" />
        ) : (
          <span
            className={`font-medium ${
              userHasLiked ? 'text-red-600' : 'text-gray-600 dark:text-gray-400'
            } group-hover/like:text-red-600`}
          >
            {likesCount}
          </span>
        )}
      </button>

      <button
        onClick={() => onToggleComments(postId, !showComments)}
        className="flex items-center gap-2 group/comment"
        disabled={isCommenting}
        aria-label="Show comments"
      >
        <div className="p-2 rounded-full bg-gray-100 dark:bg-gray-800 group-hover/comment:bg-blue-50 dark:group-hover/comment:bg-blue-900/20 transition-colors">
          <HiChatAlt
            className="w-6 h-6 text-gray-500 dark:text-gray-400 group-hover/comment:text-blue-500 transition-colors"
            aria-hidden="true"
          />
        </div>
        {isCommenting ? (
          <Loader2 className="w-5 h-5 animate-spin" aria-hidden="true" />
        ) : (
          <span className="font-medium text-gray-600 dark:text-gray-400 group-hover/comment:text-blue-600">
            {commentsCount}
          </span>
        )}
      </button>
    </div>
  );
}
