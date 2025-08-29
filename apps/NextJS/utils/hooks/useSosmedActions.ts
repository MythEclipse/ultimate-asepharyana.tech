// apps/NextJS/hooks/useSosmedActions.ts
import { useState } from 'react';
import { mutate } from 'swr';
import { useSession } from 'next-auth/react';
import { useGlobalStore } from './useGlobalStore';

export function useSosmedActions() {
  const { data: session } = useSession();



  const setNewComment = useGlobalStore((s) => s.setNewComment);

  const [isLiking, setIsLiking] = useState<Record<string, boolean>>({});
  const [isCommenting, setIsCommenting] = useState<Record<string, boolean>>({});
  const [isEditing, setIsEditing] = useState<Record<string, boolean>>({});
  const [isDeleting, setIsDeleting] = useState<Record<string, boolean>>({});

  const handleLike = async (postId: string) => {
    if (!session?.user) return;
    setIsLiking((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/likes`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId }),
      });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsLiking((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleUnlike = async (postId: string) => {
    if (!session?.user) return;
    setIsLiking((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/likes`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId }),
      });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsLiking((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleAddComment = async (postId: string, comment: string) => {
    if (!session?.user || !comment?.trim()) return;
    setIsCommenting((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content: comment, postId }),
      });
      mutate(`/api/sosmed/posts`);
      setNewComment(postId, '');
    } finally {
      setIsCommenting((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleEditPost = async (postId: string, content: string) => {
    if (!session?.user) return;
    setIsEditing((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: postId, content }),
      });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsEditing((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleDeletePost = async (postId: string) => {
    if (!session?.user) return;
    setIsDeleting((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: postId }),
      });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsDeleting((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleEditComment = async (commentId: string, content: string) => {
    if (!session?.user) return;
    setIsEditing((prev) => ({ ...prev, [commentId]: true }));
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: commentId, content }),
      });
      mutate(`/api/sosmed/posts`);
    } finally {
      setIsEditing((prev) => ({ ...prev, [commentId]: false }));
    }
  };

  const handleDeleteComment = async (commentId: string) => {
    if (!session?.user) return;
    setIsDeleting((prev) => ({ ...prev, [commentId]: true }));
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: commentId }),
      });
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
