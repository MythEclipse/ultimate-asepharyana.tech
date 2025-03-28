'use client';
import React, { useState } from 'react';
import { HiHeart, HiChatAlt, HiPencil, HiTrash, HiBadgeCheck } from 'react-icons/hi';
import Image from 'next/image';
import { Textarea } from '@/components/text/textarea';
import ButtonA from '@/components/button/NormalButton';
import { Posts, User, Likes, Comments } from '@prisma/client';
import { formatDistanceToNow } from 'date-fns';
import { useSession } from 'next-auth/react';
interface PostCardProps {
  post: Posts & {
    user: User;
    likes: Likes[];
    comments: (Comments & { user: User })[];
  };
  currentUserId: string;
  handleLike: (postId: string) => void;
  handleUnlike: (postId: string) => void;
  handleAddComment: (postId: string, comment: string) => void;
  handleEditPost: (postId: string, content: string) => void;
  handleDeletePost: (postId: string) => void;
  handleEditComment: (commentId: string, content: string) => void;
  handleDeleteComment: (commentId: string) => void;
  toggleComments: (postId: string) => void;
  showComments: boolean;
  newComment: string;
  setNewComment: (comment: string) => void;
}

export default function PostCard({
  post,
  currentUserId,
  handleLike,
  handleUnlike,
  handleAddComment,
  handleEditPost,
  handleDeletePost,
  handleEditComment,
  handleDeleteComment,
  toggleComments,
  showComments,
  newComment,
  setNewComment,
}: PostCardProps) {
  const [isEditingPost, setIsEditingPost] = useState(false);
  const [editedPostContent, setEditedPostContent] = useState(post.content);
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);
  const [editedCommentContent, setEditedCommentContent] = useState('');
  const { data: session } = useSession();

  const handleEditPostSubmit = () => {
    handleEditPost(post.id, editedPostContent);
    setIsEditingPost(false);
  };

  const handleEditCommentSubmit = (commentId: string) => {
    handleEditComment(commentId, editedCommentContent);
    setEditingCommentId(null);
  };

  const userHasLiked = post.likes.some((like) => like.userId === currentUserId);

  return (
    <div className="relative p-8 bg-white dark:bg-gray-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all border border-transparent hover:border-blue-500/20 group">
      {/* User Header */}
      <div className="flex items-start gap-4 mb-6">
        <div className="relative">
          <Image
            src={post.user.image || '/default-profile.png'}
            alt={post.user.name || 'User'}
            width={56}
            height={56}
            className="rounded-full border-2 border-blue-400/80 shadow-md hover:border-blue-500 transition-all cursor-pointer"
          />
          <div className="absolute -bottom-1 -right-1 bg-blue-500 text-white rounded-full px-2 py-0.5 text-xs shadow-sm">
            <HiBadgeCheck className="w-4 h-4" />
          </div>
        </div>

        <div className="flex-1">
          <div className="flex items-center gap-3">
            <h2 className="text-lg font-bold text-gray-800 dark:text-gray-100 hover:text-blue-600 cursor-pointer">
              {post.user.name}
            </h2>
            <span className="text-sm text-gray-500 dark:text-gray-400">
              · {formatDistanceToNow(new Date(post.created_at), { addSuffix: true })}
            </span>
          </div>
          <p className="text-sm text-gray-500 dark:text-gray-400">
            @{post.user.name?.replace(/\s+/g, '').toLowerCase()}
          </p>
        </div>

        {currentUserId === post.user.id && (
          <div className="flex space-x-3 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
              onClick={() => setIsEditingPost(true)}
              className="p-2 hover:bg-blue-50 dark:hover:bg-gray-800 rounded-full text-blue-500 hover:text-blue-600 transition-colors"
              title="Edit post"
            >
              <HiPencil className="w-5 h-5" />
            </button>
            <button
              onClick={() => handleDeletePost(post.id)}
              className="p-2 hover:bg-red-50 dark:hover:bg-gray-800 rounded-full text-red-500 hover:text-red-600 transition-colors"
              title="Delete post"
            >
              <HiTrash className="w-5 h-5" />
            </button>
          </div>
        )}
      </div>

      {/* Content Section */}
      {isEditingPost ? (
        <div className="mb-6 space-y-4">
          <Textarea
            value={editedPostContent}
            onChange={(e) => setEditedPostContent(e.target.value)}
            className="min-h-[120px] text-lg border-2 border-blue-200 focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl transition-all"
          />
          <div className="flex gap-3">
            <ButtonA
              onClick={handleEditPostSubmit}
              className="bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 text-white shadow-lg hover:shadow-xl transition-all"
            >
              Save Changes
            </ButtonA>
            <ButtonA
              onClick={() => setIsEditingPost(false)}
              className="bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
            >
              Cancel
            </ButtonA>
          </div>
        </div>
      ) : (
        <p className="text-gray-800 dark:text-gray-200 mb-6 text-lg leading-relaxed">
          {post.content}
        </p>
      )}

      {/* Media */}
      {post.image_url && (
        <div className="relative mb-6 rounded-xl overflow-hidden border border-gray-200 dark:border-gray-800">
          <Image
            src={post.image_url}
            alt="Post image"
            width={800}
            height={450}
            className="object-cover w-full hover:scale-105 transition-transform duration-300 cursor-pointer"
          />
        </div>
      )}

      {/* Engagement Bar */}
      <div className="flex items-center gap-6 border-t border-gray-100 dark:border-gray-800 pt-6">
        <button
          onClick={() => (userHasLiked ? handleUnlike(post.id) : handleLike(post.id))}
          className="flex items-center gap-2 group/like"
        >
          <div className="p-2 rounded-full bg-gray-100 dark:bg-gray-800 group-hover/like:bg-red-50 dark:group-hover/like:bg-red-900/20 transition-colors">
            <HiHeart className={`w-6 h-6 ${userHasLiked ? 'text-red-500 fill-current' : 'text-gray-500 dark:text-gray-400'} group-hover/like:text-red-500 transition-colors`} />
          </div>
          <span className={`font-medium ${userHasLiked ? 'text-red-600' : 'text-gray-600 dark:text-gray-400'} group-hover/like:text-red-600`}>
            {post.likes.length}
          </span>
        </button>

        <button
          onClick={() => toggleComments(post.id)}
          className="flex items-center gap-2 group/comment"
        >
          <div className="p-2 rounded-full bg-gray-100 dark:bg-gray-800 group-hover/comment:bg-blue-50 dark:group-hover/comment:bg-blue-900/20 transition-colors">
            <HiChatAlt className="w-6 h-6 text-gray-500 dark:text-gray-400 group-hover/comment:text-blue-500 transition-colors" />
          </div>
          <span className="font-medium text-gray-600 dark:text-gray-400 group-hover/comment:text-blue-600">
            {post.comments.length}
          </span>
        </button>
      </div>

      {/* Comments Section */}
      {showComments && (
        <div className="mt-6 space-y-6">
          {post.comments.map((comment) => (
            <div
              key={comment.id}
              className="pl-6 border-l-2 border-blue-200 dark:border-blue-900/50 relative"
            >
              <div className="absolute left-0 top-4 w-4 h-px bg-gray-200 dark:bg-gray-800" />
              <div className="flex items-start gap-3 mb-3">
                <Image
                  src={comment.user.image || '/profile-circle-svgrepo-com.svg'}
                  alt={comment.user.name || 'User'}
                  width={40}
                  height={40}
                  className="rounded-full border-2 border-blue-200 dark:border-blue-900/50"
                />
                <div className="flex-1">
                  <div className="flex items-center gap-2">
                    <strong className="text-gray-800 dark:text-gray-200">{comment.user.name}</strong>
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      {formatDistanceToNow(new Date(comment.created_at), { addSuffix: true })}
                    </span>
                  </div>
                  {editingCommentId === comment.id ? (
                    <div className="mt-2 space-y-3">
                      <Textarea
                        value={editedCommentContent}
                        onChange={(e) => setEditedCommentContent(e.target.value)}
                        className="border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-lg"
                      />
                      <div className="flex gap-3">
                        <ButtonA
                          onClick={() => handleEditCommentSubmit(comment.id)}
                          className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white text-sm"
                        >
                          Save
                        </ButtonA>
                        <ButtonA
                          onClick={() => setEditingCommentId(null)}
                          className="px-4 py-2 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm"
                        >
                          Cancel
                        </ButtonA>
                      </div>
                    </div>
                  ) : (
                    <p className="text-gray-700 dark:text-gray-300 mt-1">{comment.content}</p>
                  )}
                </div>
                {currentUserId === comment.user.id && !editingCommentId && (
                  <div className="flex space-x-2 ml-3">
                    <button
                      onClick={() => {
                        setEditingCommentId(comment.id);
                        setEditedCommentContent(comment.content);
                      }}
                      className="p-1.5 hover:bg-blue-50 dark:hover:bg-gray-800 rounded-full text-blue-500 hover:text-blue-600"
                    >
                      <HiPencil className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleDeleteComment(comment.id)}
                      className="p-1.5 hover:bg-red-50 dark:hover:bg-gray-800 rounded-full text-red-500 hover:text-red-600"
                    >
                      <HiTrash className="w-4 h-4" />
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}

          <div className="pt-6 border-t border-gray-100 dark:border-gray-800">
            <div className="flex gap-4 items-start">
              <Image
                src={session?.user?.image || '/profile-circle-svgrepo-com.svg'}
                alt="Your profile"
                width={48}
                height={48}
                className="rounded-full border-2 border-blue-200 dark:border-blue-900/50 flex-shrink-0"
              />
              <div className="flex-1 space-y-4">
                <Textarea
                  placeholder="Add a comment..."
                  value={newComment}
                  onChange={(e) => setNewComment(e.target.value)}
                  className="min-h-[100px] border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl"
                />
                <ButtonA
                  onClick={() => handleAddComment(post.id, newComment)}
                  className="bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 text-white shadow-lg hover:shadow-xl transition-all"
                >
                  Post Comment
                </ButtonA>
              </div>
            </div>
          </div>
        </div>
      )
      }
    </div >
  );
}
