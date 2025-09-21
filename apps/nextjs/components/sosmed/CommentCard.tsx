import React from 'react';
import Image from 'next/image';
import { HiPencil, HiTrash } from 'react-icons/hi';
import { formatDistanceToNow } from 'date-fns';
import { Textarea } from '../text/textarea';
import { Button } from '../ui/button';
import { Loader2 } from 'lucide-react';
import { Comments } from '@asepharyana/services';
import { ClientUser } from '../shared/types';

interface CommentCardProps {
  comment: Comments & {
    id: string;
    created_at: Date;
    updated_at: Date;
    user: ClientUser;
  };
  authenticatedUserId: string | undefined;
  editingCommentId: string | null;
  editedCommentContent: string;
  onEditCommentChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onSaveEditComment: (commentId: string) => void;
  onCancelEditComment: () => void;
  onStartEditComment: (
    comment: Comments & {
      id: string;
      created_at: Date;
      updated_at: Date;
      user: ClientUser;
    },
  ) => void;
  onDeleteComment: (commentId: string) => void;
  isEditingComment: Record<string, boolean>;
  isDeletingComment: Record<string, boolean>;
}

export default function CommentCard({
  comment,
  authenticatedUserId,
  editingCommentId,
  editedCommentContent,
  onEditCommentChange,
  onSaveEditComment,
  onCancelEditComment,
  onStartEditComment,
  onDeleteComment,
  isEditingComment,
  isDeletingComment,
}: CommentCardProps) {
  return (
    <div
      className="pl-6 border-l-2 border-blue-200 dark:border-blue-900/50 relative"
      role="region"
      aria-label={`Comment by ${comment.user.name || 'User'}`}
    >
      <div className="absolute left-0 top-4 w-4 h-px bg-gray-200 dark:bg-gray-800" />
      <div className="flex items-start gap-3">
        <Image
          src={comment.user.image || '/profile-circle-svgrepo-com.svg'}
          alt={comment.user.name || 'User profile picture'}
          width={40}
          height={40}
          unoptimized
          className="rounded-full border-2 border-blue-200 dark:border-blue-900/50"
        />
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <strong className="text-gray-800 dark:text-gray-200">
              {comment.user.name}
            </strong>
            <span className="text-sm text-gray-500 dark:text-gray-400">
              {formatDistanceToNow(new Date(comment.created_at), {
                addSuffix: true,
              })}
            </span>
          </div>
          {editingCommentId === comment.id ? (
            <div className="mt-2 space-y-3">
              <Textarea
                value={editedCommentContent}
                onChange={onEditCommentChange}
                className="border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-lg"
                aria-label="Edit comment"
              />
              <div className="flex gap-3">
                <Button
                  onClick={() => onSaveEditComment(comment.id)}
                  variant="default"
                  size="default"
                  className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white text-sm"
                  disabled={isEditingComment[comment.id]}
                  aria-label="Save comment"
                >
                  {isEditingComment[comment.id] ? (
                    <div className="flex items-center justify-center gap-2">
                      <Loader2
                        className="w-4 h-4 animate-spin"
                        aria-hidden="true"
                      />
                      <span>Saving...</span>
                    </div>
                  ) : (
                    'Save'
                  )}
                </Button>
                <Button
                  onClick={onCancelEditComment}
                  variant="secondary"
                  size="default"
                  className="px-4 py-2 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm"
                  aria-label="Cancel editing comment"
                >
                  Cancel
                </Button>
              </div>
            </div>
          ) : (
            <p className="text-gray-700 dark:text-gray-300 mt-1">
              {comment.content}
            </p>
          )}
        </div>
        {authenticatedUserId === comment.user.id &&
          editingCommentId !== comment.id && (
            <div className="flex space-x-2 ml-3">
              <button
                onClick={() => onStartEditComment(comment)}
                className="p-1.5 hover:bg-blue-50 dark:hover:bg-gray-800 rounded-full text-blue-500 hover:text-blue-600"
                disabled={isEditingComment[comment.id]}
                aria-label="Edit comment"
              >
                <HiPencil className="w-4 h-4" aria-hidden="true" />
              </button>
              <button
                onClick={() => onDeleteComment(comment.id)}
                className="p-1.5 hover:bg-red-50 dark:hover:bg-gray-800 rounded-full text-red-500 hover:text-red-600"
                disabled={isDeletingComment[comment.id]}
                aria-label="Delete comment"
              >
                <HiTrash className="w-4 h-4" aria-hidden="true" />
              </button>
            </div>
          )}
      </div>
    </div>
  );
}
