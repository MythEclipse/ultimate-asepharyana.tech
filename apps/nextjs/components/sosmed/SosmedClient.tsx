'use client';
import React, { useState } from 'react';
import PostCard from '../../components/sosmed/PostCard';
import { ThemedCard } from '../../components/ui/CardSystem';
import { Textarea } from '../../components/text/textarea';
import { Button } from '../../components/ui/button';
import { useAuth } from '../../lib/auth-context';
import { Loader2, UploadCloud, Lock } from 'lucide-react';
import useSWR, { mutate } from 'swr';
import { fetchData } from '../../utils/useFetch';
import { useSosmedActions } from '../../utils/hooks/useSosmedActions';
import type { Posts, Likes, Comments, ClientUser } from '../../types';

const fetcher = async (url: string) => {
  const response = await fetchData<{ posts: Posts[] }>(url);
  return response.data;
};

export default function SosmedClient() {
  const { user } = useAuth();
  const [content, setContent] = useState('');
  const [imageUrl, setImageUrl] = useState('');
  const [isUploading, setIsUploading] = useState(false);
  const [isPosting, setIsPosting] = useState(false);

  const {
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
  } = useSosmedActions();

  const { data: postsData } = useSWR(
    user?.email ? `/api/sosmed/posts` : null,
    fetcher,
    {
      refreshInterval: 1000,
      dedupingInterval: 1000,
      revalidateOnFocus: true,
      revalidateOnReconnect: true,
    },
  );

  const handleContentChange = (e: React.ChangeEvent<HTMLTextAreaElement>) =>
    setContent(e.target.value);

  const handlePostSubmit = async () => {
    if ((!content.trim() && !imageUrl) || isPosting) return;
    if (!user?.email) {
      console.error('User not authenticated to create post.');
      return;
    }

    setIsPosting(true);
    try {
      const request = {
        content,
        imageUrl,
      };
      const response = await fetchData(`/api/sosmed/posts`, 'POST', request);
      if (response.status && response.status >= 400) {
        throw new Error('Failed to create post');
      }
      setContent('');
      setImageUrl('');
      (document.querySelector('input[type="file"]') as HTMLInputElement).value =
        '';
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error creating post:', error);
    } finally {
      setIsPosting(false);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setIsUploading(true);
      const formData = new FormData();
      formData.append('file', file);
      fetchData<{ url: string }>(`/api/uploader`, 'POST', undefined, formData)
        .then((response) => {
          setImageUrl(response.data.url);
          setIsUploading(false);
        })
        .catch((err) => {
          console.error('Error uploading file:', err);
          setIsUploading(false);
        });
    }
  };

  return (
    <div className="container mx-auto py-8 px-4 max-w-4xl md:py-12 bg-background text-foreground">
      <h1 className="text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
        Community Hub
      </h1>

      {user ? (
        <div className="mb-8 md:mb-10 group">
          <div className="relative bg-gradient-to-br from-white/50 to-blue-50/50 dark:from-gray-800/50 dark:to-gray-900/50 rounded-2xl p-1 shadow-lg transition-all hover:shadow-xl">
            <ThemedCard>
              <div className="space-y-4">
                <Textarea
                  placeholder="Share your thoughts..."
                  value={content}
                  onChange={handleContentChange}
                  className="min-h-[120px] text-lg border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl transition-all"
                />
                <div className="flex flex-col gap-4">
                  <label className="flex flex-col items-center justify-center gap-2 p-4 border-2 border-dashed border-blue-100 dark:border-gray-700 rounded-xl cursor-pointer hover:bg-blue-50/50 dark:hover:bg-gray-800/50 transition-colors">
                    <UploadCloud className="w-8 h-8 text-blue-500" />
                    <span className="text-gray-600 dark:text-gray-400">
                      {imageUrl ? 'Image attached' : 'Attach image or video'}
                    </span>
                    <input
                      type="file"
                      onChange={handleFileChange}
                      className="hidden"
                      id="file-input"
                    />
                  </label>
                  <Button
                    onClick={handlePostSubmit}
                    disabled={
                      isUploading || isPosting || (!content.trim() && !imageUrl)
                    }
                    variant="gradient"
                    size="gradientSm"
                    className="w-full py-3.5 px-6 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold rounded-xl shadow-md transition-all hover:shadow-lg hover:scale-[1.02] disabled:opacity-70 disabled:pointer-events-none"
                  >
                    {isUploading || isPosting ? (
                      <div className="flex items-center justify-center gap-2">
                        <Loader2
                          className="w-5 h-5 animate-spin"
                          aria-hidden="true"
                        />
                        <span>
                          {isPosting ? 'Publishing...' : 'Uploading...'}
                        </span>
                      </div>
                    ) : (
                      'Publish Post'
                    )}
                  </Button>
                </div>
              </div>
            </ThemedCard>
          </div>
        </div>
      ) : (
        <div className="mb-10 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-gray-800 dark:to-gray-900/50 rounded-2xl p-6 text-center space-y-4">
          <div className="flex flex-col items-center gap-4">
            <Lock className="w-10 h-10 text-blue-500" />
            <h3 className="text-xl font-semibold text-gray-800 dark:text-gray-100">
              Join the Conversation
            </h3>
            <p className="text-gray-600 dark:text-gray-400 mb-4">
              Sign in to share your thoughts and engage with the community
            </p>
            <div className="w-full max-w-[150px] sm:max-w-[200px]">
              <Button
                href="/login"
                variant="gradient"
                size="gradientSm"
                className="w-full bg-blue-600 hover:bg-blue-700 text-white"
              >
                Get Started
              </Button>
            </div>
          </div>
        </div>
      )}

      <div className="grid gap-8">
        {postsData?.posts?.map(
          (
            post: Posts & {
              user: ClientUser;
              likes: Likes[];
              comments: Comments[];
            },
          ) => (
            <PostCard
              key={post.id}
              post={{
                ...post,
                user: post.user || {
                  id: '',
                  name: null,
                  email: null,
                  image: null,
                  emailVerified: null,
                  role: '',
                },
                likes: post.likes || [],
                comments:
                  post.comments?.map(
                    (comment: Comments & { user?: ClientUser }) => ({
                      ...comment,
                      user: comment.user || {
                        id: '',
                        name: null,
                        email: null,
                        image: null,
                        emailVerified: null,
                        role: '',
                      },
                    }),
                  ) || [],
              }}
              handleLike={handleLike}
              handleUnlike={handleUnlike}
              handleAddComment={handleAddComment}
              handleEditPost={handleEditPost}
              handleDeletePost={handleDeletePost}
              handleEditComment={handleEditComment}
              handleDeleteComment={handleDeleteComment}
              isLiking={isLiking}
              isCommenting={isCommenting}
              isEditingPost={isEditing}
              isDeletingPost={isDeleting}
              isEditingComment={isEditing}
              isDeletingComment={isDeleting}
            />
          ),
        )}
      </div>
    </div>
  );
}
