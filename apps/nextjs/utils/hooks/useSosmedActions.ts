import { useState } from 'react';
import { mutate } from 'swr';
import { useAuth } from '../../lib/auth-context';
import { useGlobalStore } from './useGlobalStore';
import { SosmedService } from '../../api/sosmed';

export function useSosmedActions() {
  const { user } = useAuth();
  const setNewComment = useGlobalStore((s) => s.setNewComment);

  const [isLiking, setIsLiking] = useState<Record<string, boolean>>({});
  const [isCommenting, setIsCommenting] = useState<Record<string, boolean>>({});
  const [isEditing, setIsEditing] = useState<Record<string, boolean>>({});
  const [isDeleting, setIsDeleting] = useState<Record<string, boolean>>({});

  const handleLike = async (postId: string) => {
    if (!user) throw new Error('Authentication required');
    setIsLiking((prev) => ({ ...prev, [postId]: true }));
    try {
      await SosmedService.likePost(postId);
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsLiking((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleUnlike = async (postId: string) => {
    if (!user) throw new Error('Authentication required');
    setIsLiking((prev) => ({ ...prev, [postId]: true }));
    try {
      await SosmedService.unlikePost(postId);
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsLiking((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleAddComment = async (postId: string, comment: string) => {
    if (!user) throw new Error('Authentication required');
    if (!comment?.trim()) return;
    setIsCommenting((prev) => ({ ...prev, [postId]: true }));
    try {
      await SosmedService.addComment({ content: comment, postId });
      mutate(`/api/sosmed/posts`);
      setNewComment(postId, '');
    } finally {
      setIsCommenting((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleEditPost = async (postId: string, content: string) => {
    if (!user) throw new Error('Authentication required');
    setIsEditing((prev) => ({ ...prev, [postId]: true }));
    try {
      await SosmedService.updatePost(postId, { content });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsEditing((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleDeletePost = async (postId: string) => {
    if (!user) throw new Error('Authentication required');
    setIsDeleting((prev) => ({ ...prev, [postId]: true }));
    try {
      await SosmedService.deletePost(postId);
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsDeleting((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleEditComment = async (commentId: string, content: string) => {
    if (!user) throw new Error('Authentication required');
    setIsEditing((prev) => ({ ...prev, [commentId]: true }));
    try {
      await SosmedService.updateComment(commentId, { content });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsEditing((prev) => ({ ...prev, [commentId]: false }));
    }
  };

  const handleDeleteComment = async (commentId: string) => {
    if (!user) throw new Error('Authentication required');
    setIsDeleting((prev) => ({ ...prev, [commentId]: true }));
    try {
      await SosmedService.deleteComment(commentId);
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsDeleting((prev) => ({ ...prev, [commentId]: false }));
    }
  };

  return {
    isLiking,
    isCommenting,
    isEditing,
    isDeleting,
    handleLike,
    handleUnlike,
    handleAddComment,
    handleEditPost,
    handleDeletePost,
    handleEditComment,
    handleDeleteComment,
  };
}
