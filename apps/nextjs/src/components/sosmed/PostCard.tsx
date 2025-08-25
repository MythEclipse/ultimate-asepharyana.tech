// apps/NextJS/components/sosmed/PostCard.tsx
'use client';
import React, { useState } from 'react';
import {
  HiHeart,
  HiChatAlt,
  HiPencil,
  HiTrash,
  HiBadgeCheck,
} from 'react-icons/hi';
import Image from 'next/image';
import { Textarea } from '../text/textarea';
import ButtonA from '../ui/BaseButton';
import { Posts, Comments, Likes } from '@asepharyana/database';
import { formatDistanceToNow } from 'date-fns';
import { useSession } from 'next-auth/react';
import { Loader2 } from 'lucide-react';
import { useGlobalStore } from '../../utils/hooks/useGlobalStore';

interface ClientUser {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
  emailVerified: Date | null;
  role: string;
}

interface PostCardProps {
  readonly post: Posts & {
    readonly user: ClientUser;
    readonly likes: readonly Likes[];
    readonly comments: readonly (Comments & { readonly user: ClientUser })[];
  };
  readonly handleLike: (postId: string) => void;
  readonly handleUnlike: (postId: string) => void;
  readonly handleAddComment: (postId: string, comment: string) => void;
  readonly handleEditPost: (postId: string, content: string) => void;
  readonly handleDeletePost: (postId: string) => void;
  readonly handleEditComment: (commentId: string, content: string) => void;
  readonly handleDeleteComment: (commentId: string) => void;
  readonly isLiking: Record<string, boolean>;
  readonly isCommenting: Record<string, boolean>;
  readonly isEditing: Record<string, boolean>;
  readonly isDeleting: Record<string, boolean>;
}

export default function PostCard({
  post,
  handleLike,
  handleUnlike,
  handleAddComment,
  handleEditPost,
  handleDeletePost,
  handleEditComment,
  handleDeleteComment,
  isLiking,
  isCommenting,
  isEditing,
  isDeleting,
}: PostCardProps) {
  const [isEditingPost, setIsEditingPost] = useState(false);
  const [editedPostContent, setEditedPostContent] = useState(post.content);
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);
  const [editedCommentContent, setEditedCommentContent] = useState('');
  const { data: session } = useSession();

  // Zustand global state for Sosmed UI
  const showComments = useGlobalStore((s) => s.showComments[post.id] || false);
  const setShowComments = useGlobalStore((s) => s.setShowComments);
  const newComment = useGlobalStore((s) => s.newComments[post.id] || '');
  const setNewComment = useGlobalStore((s) => s.setNewComment);

  const authenticatedUserId = session?.user?.id;

  const handleEditPostSubmit = () => {
    handleEditPost(post.id, editedPostContent);
    setIsEditingPost(false);
  };

  const handleEditCommentSubmit = (commentId: string) => {
    handleEditComment(commentId, editedCommentContent);
    setEditingCommentId(null);
  };

  const userHasLiked = post.likes.some((like: Likes) => like.userId === authenticatedUserId);

  return (
    <div
      className='relative p-8 bg-white dark:bg-gray-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all border border-transparent hover:border-blue-500/20 group'
      role="region"
      aria-label={`Post by ${post.user.name || 'User'}`}
    >
      {/* User Header */}
      <div className='flex items-start gap-4 mb-6'>
        <div className='relative'>
          <Image
            src={post.user.image || '/default-profile.png'}
            alt={post.user.name || 'User profile picture'}
            width={56}
            height={56}
            className='rounded-full border-2 border-blue-400/80 shadow-md hover:border-blue-500 transition-all cursor-pointer'
          />
          <div className='absolute -bottom-1 -right-1 bg-blue-500 text-white rounded-full px-2 py-0.5 text-xs shadow-sm'>
            <HiBadgeCheck className='w-4 h-4' aria-hidden="true" />
          </div>
        </div>

        <div className='flex-1'>
          <div className='flex items-center gap-3'>
            <h2 className='text-lg font-bold text-gray-800 dark:text-gray-100 hover:text-blue-600 cursor-pointer'>
              {post.user.name}
            </h2>
            <span className='text-sm text-gray-500 dark:text-gray-400'>
              ·{' '}
              {formatDistanceToNow(new Date(post.created_at), {
                addSuffix: true,
              })}
            </span>
          </div>
          <p className='text-sm text-gray-500 dark:text-gray-400'>
            @{post.user.name?.replace(/\s+/g, '').toLowerCase()}
          </p>
        </div>

        {authenticatedUserId === post.user.id && (
          <div className='flex space-x-3 opacity-0 group-hover:opacity-100 transition-opacity'>
            <button
              onClick={() => setIsEditingPost(true)}
              className='p-2 hover:bg-blue-50 dark:hover:bg-gray-800 rounded-full text-blue-500 hover:text-blue-600 transition-colors'
              title='Edit post'
              aria-label="Edit post"
              disabled={isEditing[post.id]}
            >
              <HiPencil className='w-5 h-5' aria-hidden="true" />
            </button>
            <button
              onClick={() => handleDeletePost(post.id)}
              className='p-2 hover:bg-red-50 dark:hover:bg-gray-800 rounded-full text-red-500 hover:text-red-600 transition-colors'
              title='Delete post'
              aria-label="Delete post"
              disabled={isDeleting[post.id]}
            >
              <HiTrash className='w-5 h-5' aria-hidden="true" />
            </button>
          </div>
        )}
      </div>

      {/* Content Section */}
      {isEditingPost ? (
        <div className='mb-6 space-y-4'>
          <Textarea
            value={editedPostContent}
            onChange={(e) => setEditedPostContent(e.target.value)}
            className='min-h-[120px] text-lg border-2 border-blue-200 focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl transition-all'
            aria-label="Edit post content"
          />
          <div className='flex gap-3'>
            <ButtonA
              onClick={handleEditPostSubmit}
              className='bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 text-white shadow-lg hover:shadow-xl transition-all'
              disabled={isEditing[post.id]}
              aria-label="Save changes"
            >
              {isEditing[post.id] ? (
                <div className='flex items-center justify-center gap-2'>
                  <Loader2 className='w-5 h-5 animate-spin' aria-hidden="true" />
                  <span>Saving...</span>
                </div>
              ) : (
                'Save Changes'
              )}
            </ButtonA>
            <ButtonA
              onClick={() => setIsEditingPost(false)}
              className='bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'
              aria-label="Cancel editing"
            >
              Cancel
            </ButtonA>
          </div>
        </div>
      ) : (
        <p className='text-gray-800 dark:text-gray-200 mb-6 text-lg leading-relaxed'>
          {post.content}
        </p>
      )}

      {/* Media */}
      {post.image_url && (
        <div className='relative mb-6 rounded-xl overflow-hidden border border-gray-200 dark:border-gray-800'>
          <Image
            src={post.image_url}
            alt='Post image'
            width={800}
            unoptimized
            height={450}
            className='object-cover w-full hover:scale-105 transition-transform duration-300 cursor-pointer'
          />
        </div>
      )}

      {/* Engagement Bar */}
      <div className='flex items-center gap-6 border-t border-gray-100 dark:border-gray-800 pt-6'>
        <button
          onClick={() =>
            userHasLiked ? handleUnlike(post.id) : handleLike(post.id)
          }
          className='flex items-center gap-2 group/like'
          disabled={isLiking[post.id]}
          aria-label={userHasLiked ? "Unlike post" : "Like post"}
          aria-pressed={userHasLiked}
        >
          <div className='p-2 rounded-full bg-gray-100 dark:bg-gray-800 group-hover/like:bg-red-50 dark:group-hover/like:bg-red-900/20 transition-colors'>
            <HiHeart
              className={`w-6 h-6 ${userHasLiked ? 'text-red-500 fill-current' : 'text-gray-500 dark:text-gray-400'} group-hover/like:text-red-500 transition-colors`}
              aria-hidden="true"
            />
          </div>
          {isLiking[post.id] ? (
            <Loader2 className='w-5 h-5 animate-spin' aria-hidden="true" />
          ) : (
            <span
              className={`font-medium ${userHasLiked ? 'text-red-600' : 'text-gray-600 dark:text-gray-400'} group-hover/like:text-red-600`}
            >
              {post.likes.length}
            </span>
          )}
        </button>

        <button
          onClick={() => setShowComments(post.id, !showComments)}
          className='flex items-center gap-2 group/comment'
          disabled={isCommenting[post.id]}
          aria-label="Show comments"
        >
          <div className='p-2 rounded-full bg-gray-100 dark:bg-gray-800 group-hover/comment:bg-blue-50 dark:group-hover/comment:bg-blue-900/20 transition-colors'>
            <HiChatAlt className='w-6 h-6 text-gray-500 dark:text-gray-400 group-hover/comment:text-blue-500 transition-colors' aria-hidden="true" />
          </div>
          {isCommenting[post.id] ? (
            <Loader2 className='w-5 h-5 animate-spin' aria-hidden="true" />
          ) : (
            <span className='font-medium text-gray-600 dark:text-gray-400 group-hover/comment:text-blue-600'>
              {post.comments.length}
            </span>
          )}
        </button>
      </div>

      {/* Comments Section */}
      {showComments && (
        <div className='mt-6 space-y-6'>
          {post.comments.map((comment: Comments & { user: ClientUser }) => (
            <div
              key={comment.id}
              className='pl-6 border-l-2 border-blue-200 dark:border-blue-900/50 relative'
              role="region"
              aria-label={`Comment by ${comment.user.name || 'User'}`}
            >
              <div className='absolute left-0 top-4 w-4 h-px bg-gray-200 dark:bg-gray-800' />
              <div className='flex items-start gap-3 mb-3'>
                <Image
                  src={comment.user.image || '/profile-circle-svgrepo-com.svg'}
                  alt={comment.user.name || 'User profile picture'}
                  width={40}
                  height={40}
                  unoptimized
                  className='rounded-full border-2 border-blue-200 dark:border-blue-900/50'
                />
                <div className='flex-1'>
                  <div className='flex items-center gap-2'>
                    <strong className='text-gray-800 dark:text-gray-200'>
                      {comment.user.name}
                    </strong>
                    <span className='text-sm text-gray-500 dark:text-gray-400'>
                      {formatDistanceToNow(new Date(comment.created_at), {
                        addSuffix: true,
                      })}
                    </span>
                  </div>
                  {editingCommentId === comment.id ? (
                    <div className='mt-2 space-y-3'>
                      <Textarea
                        value={editedCommentContent}
                        onChange={(e) =>
                          setEditedCommentContent(e.target.value)
                        }
                        className='border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-lg'
                        aria-label="Edit comment"
                      />
                      <div className='flex gap-3'>
                        <ButtonA
                          onClick={() => handleEditCommentSubmit(comment.id)}
                          className='px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white text-sm'
                          disabled={isEditing[comment.id]}
                          aria-label="Save comment"
                        >
                          {isEditing[comment.id] ? (
                            <div className='flex items-center justify-center gap-2'>
                              <Loader2 className='w-4 h-4 animate-spin' aria-hidden="true" />
                              <span>Saving...</span>
                            </div>
                          ) : (
                            'Save'
                          )}
                        </ButtonA>
                        <ButtonA
                          onClick={() => setEditingCommentId(null)}
                          className='px-4 py-2 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm'
                          aria-label="Cancel editing comment"
                        >
                          Cancel
                        </ButtonA>
                      </div>
                    </div>
                  ) : (
                    <p className='text-gray-700 dark:text-gray-300 mt-1'>
                      {comment.content}
                    </p>
                  )}
                </div>
                {authenticatedUserId === comment.user.id && !editingCommentId && (
                  <div className='flex space-x-2 ml-3'>
                    <button
                      onClick={() => {
                        setEditingCommentId(comment.id);
                        setEditedCommentContent(comment.content);
                      }}
                      className='p-1.5 hover:bg-blue-50 dark:hover:bg-gray-800 rounded-full text-blue-500 hover:text-blue-600'
                      disabled={isEditing[comment.id]}
                      aria-label="Edit comment"
                    >
                      <HiPencil className='w-4 h-4' aria-hidden="true" />
                    </button>
                    <button
                      onClick={() => handleDeleteComment(comment.id)}
                      className='p-1.5 hover:bg-red-50 dark:hover:bg-gray-800 rounded-full text-red-500 hover:text-red-600'
                      disabled={isDeleting[comment.id]}
                      aria-label="Delete comment"
                    >
                      <HiTrash className='w-4 h-4' aria-hidden="true" />
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}

          <div className='pt-6 border-t border-gray-100 dark:border-gray-800'>
            <div className='flex gap-4 items-start'>
              <Image
                src={session?.user?.image || '/profile-circle-svgrepo-com.svg'}
                alt='Your profile'
                width={48}
                height={48}
                unoptimized
                className='rounded-full border-2 border-blue-200 dark:border-blue-900/50 flex-shrink-0'
              />
              <div className='flex-1 space-y-4'>
                <Textarea
                  placeholder='Add a comment...'
                  value={newComment}
                  onChange={(e) => setNewComment(post.id, e.target.value)}
                  className='min-h-[100px] border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl'
                  aria-label="Add a comment"
                />
                <ButtonA
                  onClick={() => handleAddComment(post.id, newComment)}
                  className='bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 text-white shadow-lg hover:shadow-xl transition-all'
                  disabled={isCommenting[post.id]}
                  aria-label="Post comment"
                >
                  {isCommenting[post.id] ? (
                    <div className='flex items-center justify-center gap-2'>
                      <Loader2 className='w-5 h-5 animate-spin' aria-hidden="true" />
                      <span>Commenting...</span>
                    </div>
                  ) : (
                    'Post Comment'
                  )}
                </ButtonA>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
