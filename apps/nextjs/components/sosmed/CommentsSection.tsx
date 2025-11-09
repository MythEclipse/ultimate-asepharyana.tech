import React from 'react';
import Image from 'next/image';
import { Textarea } from '../text/textarea';
import { Button } from '../ui/button';
import { Loader2 } from 'lucide-react';
import CommentCard from './CommentCard';
import { Comment } from '@asepharyana/services';

interface ClientUser {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
  emailVerified: Date | null;
  role: string;
}

interface CommentsSectionProps {
  comments: (Comment & {
    id: string;
    created_at: Date;
    updated_at: Date;
    user: ClientUser;
  })[];
  authenticatedUserId: string | undefined;
  newComment: string;
  onNewCommentChange: (value: string) => void;
  onAddComment: () => void;
  isCommenting: boolean;
  editingCommentId: string | null;
  editedCommentContent: string;
  onEditCommentChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onSaveEditComment: (commentId: string) => void;
  onCancelEditComment: () => void;
  onStartEditComment: (
    comment: Comment & {
      id: string;
      created_at: Date;
      updated_at: Date;
      user: ClientUser;
    },
  ) => void;
  onDeleteComment: (commentId: string) => void;
  isEditingComment: Record<string, boolean>;
  isDeletingComment: Record<string, boolean>;
  sessionUserImage: string | null | undefined;
}

export default function CommentsSection({
  comments,
  authenticatedUserId,
  newComment,
  onNewCommentChange,
  onAddComment,
  isCommenting,
  editingCommentId,
  editedCommentContent,
  onEditCommentChange,
  onSaveEditComment,
  onCancelEditComment,
  onStartEditComment,
  onDeleteComment,
  isEditingComment,
  isDeletingComment,
  sessionUserImage,
}: CommentsSectionProps) {
  return (
    <div className="mt-6 space-y-6">
      {comments.map((comment) => (
        <CommentCard
          key={comment.id}
          comment={comment}
          authenticatedUserId={authenticatedUserId}
          editingCommentId={editingCommentId}
          editedCommentContent={editedCommentContent}
          onEditCommentChange={onEditCommentChange}
          onSaveEditComment={onSaveEditComment}
          onCancelEditComment={onCancelEditComment}
          onStartEditComment={onStartEditComment}
          onDeleteComment={onDeleteComment}
          isEditingComment={isEditingComment}
          isDeletingComment={isDeletingComment}
        />
      ))}

      <div className="pt-6 border-t border-gray-100 dark:border-gray-800">
        <div className="flex gap-4 items-start">
          <Image
            src={sessionUserImage || '/profile-circle-svgrepo-com.svg'}
            alt="Your profile"
            width={48}
            height={48}
            unoptimized
            className="rounded-full border-2 border-blue-200 dark:border-blue-900/50 flex-shrink-0"
          />
          <div className="flex-1 space-y-4">
            <Textarea
              placeholder="Add a comment..."
              value={newComment}
              onChange={(e) => onNewCommentChange(e.target.value)}
              className="min-h-[100px] border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl"
              aria-label="Add a comment"
            />
            <Button
              variant="gradient"
              size="gradientSm"
              onClick={onAddComment}
              disabled={isCommenting}
              aria-label="Post comment"
            >
              {isCommenting ? (
                <div className="flex items-center justify-center gap-2">
                  <Loader2
                    className="w-5 h-5 animate-spin"
                    aria-hidden="true"
                  />
                  <span>Commenting...</span>
                </div>
              ) : (
                'Post Comment'
              )}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
