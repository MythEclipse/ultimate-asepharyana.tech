'use client';
import React, { useMemo, useState } from 'react';
import { Posts, Comments, Likes } from '@asepharyana/services';
import { useSession } from 'next-auth/react';
import { useGlobalStore } from '../../utils/hooks/useGlobalStore';
import PostHeader from './PostHeader';
import PostContent from './PostContent';
import PostActions from './PostActions';
import CommentsSection from './CommentsSection';
import { ClientUser } from '../../types/ClientUser';

interface PostCardProps {
  readonly post: Posts & {
    id: string;
    created_at: Date;
    updated_at: Date;
    readonly user: ClientUser;
    readonly likes: readonly (Likes & { postId: string; userId: string })[];
    readonly comments: readonly (Comments & {
      id: string;
      created_at: Date;
      updated_at: Date;
      user: ClientUser;
    })[];
  };
  readonly handleLike: (postId: string) => void;
  readonly handleUnlike: (postId: string) => void;
  readonly handleAddComment: (postId: string, comment: string) => Promise<void>;
  readonly handleEditPost: (postId: string, content: string) => void;
  readonly handleDeletePost: (postId: string) => void;
  readonly handleEditComment: (commentId: string, content: string) => void;
  readonly handleDeleteComment: (commentId: string) => void;
  readonly isLiking: Record<string, boolean>;
  readonly isCommenting: Record<string, boolean>;
  readonly isEditingPost: Record<string, boolean>;
  readonly isDeletingPost: Record<string, boolean>;
  readonly isEditingComment: Record<string, boolean>;
  readonly isDeletingComment: Record<string, boolean>;
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
  isEditingPost: isEditingPostProp,
  isDeletingPost,
  isEditingComment,
  isDeletingComment,
}: PostCardProps) {
  const [isEditingPost, setIsEditingPost] = useState(false);
  const [editedPostContent, setEditedPostContent] = useState(post.content);
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);
  const [editedCommentContent, setEditedCommentContent] = useState('');
  const { data: session } = useSession();

  const showComments = useGlobalStore((s) => s.showComments[post.id] || false);
  const setShowComments = useGlobalStore((s) => s.setShowComments);
  const newComment = useGlobalStore((s) => s.newComments[post.id] || '');
  const setNewComment = useGlobalStore((s) => s.setNewComment);

  const authenticatedUserId = session?.user?.id;

  const userHasLiked = useMemo(
    () => post.likes.some((like) => like.userId === authenticatedUserId),
    [post.likes, authenticatedUserId],
  );

  const handleEditPostSubmit = () => {
    handleEditPost(post.id, editedPostContent);
    setIsEditingPost(false);
  };

  const handleEditCommentSubmit = (commentId: string) => {
    handleEditComment(commentId, editedCommentContent);
    setEditingCommentId(null);
  };

  const handleCommentSubmit = async () => {
    if (!newComment.trim()) return;
    await handleAddComment(post.id, newComment);
    setNewComment(post.id, '');
  };

  return (
    <article
      className="relative p-8 bg-white dark:bg-gray-900 rounded-2xl shadow-xl hover:shadow-2xl transition-all border border-transparent hover:border-blue-500/20 group"
      aria-labelledby={`post-author-${post.id}`}
    >
      <PostHeader
        postId={post.id}
        userName={post.user.name}
        userImage={post.user.image}
        createdAt={post.created_at}
        authenticatedUserId={authenticatedUserId}
        postUserId={post.user.id}
        onEditPost={() => setIsEditingPost(true)}
        onDeletePost={() => handleDeletePost(post.id)}
        isEditingPost={isEditingPostProp[post.id]}
        isDeletingPost={isDeletingPost[post.id]}
      />

      <PostContent
        content={post.content}
        imageUrl={post.image_url}
        isEditing={isEditingPost}
        editedContent={editedPostContent}
        onContentChange={(e) => setEditedPostContent(e.target.value)}
        onSave={handleEditPostSubmit}
        onCancelEdit={() => setIsEditingPost(false)}
        isSaving={isEditingPostProp[post.id]}
      />

      <PostActions
        postId={post.id}
        likesCount={post.likes.length}
        commentsCount={post.comments.length}
        userHasLiked={userHasLiked}
        onLike={handleLike}
        onUnlike={handleUnlike}
        onToggleComments={setShowComments}
        isLiking={isLiking[post.id]}
        isCommenting={isCommenting[post.id]}
        showComments={showComments}
      />

      {showComments && (
        <CommentsSection
          comments={
            post.comments as (Comments & {
              id: string;
              created_at: Date;
              updated_at: Date;
              user: ClientUser;
            })[]
          }
          authenticatedUserId={authenticatedUserId}
          newComment={newComment}
          onNewCommentChange={(value) => setNewComment(post.id, value)}
          onAddComment={handleCommentSubmit}
          isCommenting={isCommenting[post.id]}
          editingCommentId={editingCommentId}
          editedCommentContent={editedCommentContent}
          onEditCommentChange={(e) => setEditedCommentContent(e.target.value)}
          onSaveEditComment={handleEditCommentSubmit}
          onCancelEditComment={() => setEditingCommentId(null)}
          onStartEditComment={(comment) => {
            setEditingCommentId(comment.id);
            setEditedCommentContent(comment.content);
          }}
          onDeleteComment={handleDeleteComment}
          isEditingComment={isEditingComment}
          isDeletingComment={isDeletingComment}
          sessionUserImage={session?.user?.image}
        />
      )}
    </article>
  );
}
