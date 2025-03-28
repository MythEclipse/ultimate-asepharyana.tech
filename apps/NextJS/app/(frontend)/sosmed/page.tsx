'use client';
export const dynamic = 'force-dynamic';

import React, { useState } from 'react';
import PostCard from '@/components/sosmed/PostCard';
import Card from '@/components/card/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import { Posts, User, Likes, Comments } from '@prisma/client';
import { useSession } from 'next-auth/react';
import useSWR, { mutate } from 'swr';
import ButtonA from '@/components/button/NormalButton';
import { UploadCloud, Loader2, Lock } from 'lucide-react';
const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function PostPage() {
  const { data: session } = useSession();
  const { data: postsData } = useSWR(`/api/sosmed/posts`, fetcher, {
    refreshInterval: 1000,
    dedupingInterval: 1000,
    revalidateOnFocus: true,
    revalidateOnReconnect: true,
  });

  const [content, setContent] = useState('');
  const [imageUrl, setImageUrl] = useState('');
  const [isUploading, setIsUploading] = useState(false);
  const [newComments, setNewComments] = useState<Record<string, string>>({});
  const [showComments, setShowComments] = useState<Record<string, boolean>>({});

  const handleContentChange = (e: React.ChangeEvent<HTMLTextAreaElement>) =>
    setContent(e.target.value);

  const handlePostSubmit = async () => {
    if (!content.trim() && !imageUrl) return;

    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content, imageUrl }),
      });
      setContent('');
      setImageUrl('');
      (document.querySelector('input[type="file"]') as HTMLInputElement).value =
        '';
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error creating post:', error);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setIsUploading(true);
      const formData = new FormData();
      formData.append('file', file);

      fetch(`/api/uploader`, {
        method: 'POST',
        body: formData,
      })
        .then((res) => res.json())
        .then((data) => {
          setImageUrl(data.url);
          setIsUploading(false);
        })
        .catch((err) => {
          console.error('Error uploading file:', err);
          setIsUploading(false);
        });
    }
  };

  const handleLike = async (postId: string) => {
    try {
      await fetch(`/api/sosmed/likes`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error liking post:', error);
    }
  };

  const handleUnlike = async (postId: string) => {
    try {
      await fetch(`/api/sosmed/likes`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error unliking post:', error);
    }
  };

  const handleAddComment = async (postId: string) => {
    if (!newComments[postId]?.trim()) return;

    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content: newComments[postId], postId }),
      });
      mutate(`/api/sosmed/posts`);
      setNewComments((prev) => ({ ...prev, [postId]: '' }));
    } catch (error) {
      console.error('Error adding comment:', error);
    }
  };

  const handleEditPost = async (postId: string, content: string) => {
    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: postId, content }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error editing post:', error);
    }
  };

  const handleDeletePost = async (postId: string) => {
    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: postId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error deleting post:', error);
    }
  };

  const handleEditComment = async (commentId: string, content: string) => {
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: commentId, content }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error editing comment:', error);
    }
  };

  const handleDeleteComment = async (commentId: string) => {
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: commentId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error deleting comment:', error);
    }
  };

  const toggleComments = (postId: string) =>
    setShowComments((prev) => ({ ...prev, [postId]: !prev[postId] }));

  return (
    <div className='container mx-auto py-8 px-4 max-w-4xl'>
      {/* Header Section */}
      <div className='mb-12 text-center space-y-2'>
        <h1 className='text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-500 bg-clip-text text-transparent'>
          Community Hub
        </h1>
        <div className='h-1 w-24 mx-auto bg-gradient-to-r from-blue-400 to-purple-400 rounded-full' />
      </div>

      {/* Create Post Section */}
      {session ? (
        <div className='mb-10 group'>
          <div className='relative bg-gradient-to-br from-white/50 to-blue-50/50 dark:from-gray-800/50 dark:to-gray-900/50 rounded-2xl p-1 shadow-lg transition-all hover:shadow-xl'>
            <Card
            // className="!bg-transparent !border-none"
            >
              <div className='space-y-4'>
                <Textarea
                  placeholder='Share your thoughts...'
                  value={content}
                  onChange={handleContentChange}
                  className='min-h-[120px] text-lg border-2 border-blue-100 dark:border-gray-700 hover:border-blue-200 focus:border-blue-400 focus:ring-2 focus:ring-blue-200 rounded-xl transition-all'
                />

                <div className='flex flex-col gap-4'>
                  <label className='flex flex-col items-center justify-center gap-2 p-4 border-2 border-dashed border-blue-100 dark:border-gray-700 rounded-xl cursor-pointer hover:bg-blue-50/50 dark:hover:bg-gray-800/50 transition-colors'>
                    <UploadCloud className='w-8 h-8 text-blue-500' />
                    <span className='text-gray-600 dark:text-gray-300'>
                      {File ? File.name : 'Attach image or video'}
                    </span>
                    <input
                      type='file'
                      onChange={handleFileChange}
                      className='hidden'
                    />
                  </label>

                  <button
                    onClick={handlePostSubmit}
                    disabled={isUploading || !content.trim()}
                    className='w-full py-3.5 px-6 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-semibold rounded-xl shadow-md transition-all hover:shadow-lg hover:scale-[1.02] disabled:opacity-70 disabled:pointer-events-none'
                  >
                    {isUploading ? (
                      <div className='flex items-center justify-center gap-2'>
                        <Loader2 className='w-5 h-5 animate-spin' />
                        <span>Sharing...</span>
                      </div>
                    ) : (
                      'Publish Post'
                    )}
                  </button>
                </div>
              </div>
            </Card>
          </div>
        </div>
      ) : (
        <div className='mb-10 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-gray-800 dark:to-gray-900/50 rounded-2xl p-6 text-center'>
          <div className='flex flex-col items-center gap-4'>
            <Lock className='w-10 h-10 text-blue-500' />
            <h3 className='text-xl font-semibold text-gray-800 dark:text-gray-100'>
              Join the Conversation
            </h3>
            <p className='text-gray-600 dark:text-gray-400 mb-4'>
              Sign in to share your thoughts and engage with the community
            </p>
            <div className='w-full max-w-[200px]'>
              <ButtonA
                href='/login'
                className='w-full bg-blue-600 hover:bg-blue-700 text-white'
              >
                Get Started
              </ButtonA>
            </div>
          </div>
        </div>
      )}

      <div className='grid gap-8'>
        {postsData?.posts?.map(
          (
            post: Posts & {
              user?: User;
              likes?: Likes[];
              comments?: (Comments & { user?: User })[];
            }
          ) => (
            <PostCard
              key={post.id}
              post={{
                ...post,
                user: post.user || {
                  name: null,
                  id: '',
                  email: null,
                  role: '',
                  image: null,
                  emailVerified: null,
                },
                likes: post.likes || [],
                comments:
                  post.comments?.map((comment: Comments & { user?: User }) => ({
                    ...comment,
                    user: comment.user || {
                      name: null,
                      id: '',
                      email: null,
                      role: '',
                      image: null,
                      emailVerified: null,
                    },
                  })) || [],
              }}
              currentUserId={session?.user?.id ?? ''}
              handleLike={handleLike}
              handleUnlike={handleUnlike}
              handleAddComment={handleAddComment}
              handleEditPost={handleEditPost}
              handleDeletePost={handleDeletePost}
              handleEditComment={handleEditComment}
              handleDeleteComment={handleDeleteComment}
              toggleComments={toggleComments}
              showComments={!!showComments[post.id]}
              newComment={newComments[post.id] || ''}
              setNewComment={(value) =>
                setNewComments((prev) => ({ ...prev, [post.id]: value }))
              }
            />
          )
        )}
      </div>
    </div>
  );
}
