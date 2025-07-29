'use client';
import React, { useState } from 'react';
import PostCard from '@/components/sosmed/PostCard';
import Card from '@core/ui/ThemedCard';
import { Textarea } from '@/components/text/textarea';
import ButtonA from '@core/ui/BaseButton';
import { useSession } from 'next-auth/react';
import { Loader2, UploadCloud, Lock } from 'lucide-react';
import { useGlobalStore } from '@hooks/useGlobalStore';
import useSWR, { mutate } from 'swr';

// Define missing types locally with corrected field names to match API response
interface Posts {
  id: string;
  content: string;
  userId: string;
  postId: string;
  created_at: Date;
  updated_at: Date;
  authorId: string;
  image_url: string | null; // Added missing image_url field
  user: ClientUser;
  likes: Likes[];
  comments: Comments[];
}

interface Likes {
  id: string;
  userId: string;
  postId: string;
  created_at: Date;
}

interface Comments {
  id: string;
  content: string;
  userId: string;
  postId: string;
  created_at: Date;
  updated_at: Date;
  authorId: string;
  user: ClientUser;
}

interface ClientUser {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
  emailVerified: Date | null;
  role: string;
}


const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function PostPage() {
  const { data: session } = useSession();
  const [content, setContent] = useState('');
  const [imageUrl, setImageUrl] = useState('');
  const [isUploading, setIsUploading] = useState(false);
  const [isPosting, setIsPosting] = useState(false);
  const [isLiking, setIsLiking] = useState<Record<string, boolean>>({});
  const [isCommenting, setIsCommenting] = useState<Record<string, boolean>>({});
  const [isEditing, setIsEditing] = useState<Record<string, boolean>>({});
  const [isDeleting, setIsDeleting] = useState<Record<string, boolean>>({});

  const { data: postsData } = useSWR(
    session?.user?.email ? `/api/sosmed/posts` : null,
    fetcher,
    {
      refreshInterval: 1000,
      dedupingInterval: 1000,
      revalidateOnFocus: true,
      revalidateOnReconnect: true,
    }
  );

  // Correctly implement setNewComment using the global store
  const globalStore = useGlobalStore();
  const setNewComment = (postId: string, value: string) => {
    globalStore.setNewComment(postId, value);
  };

  const handleContentChange = (e: React.ChangeEvent<HTMLTextAreaElement>) =>
    setContent(e.target.value);

  const handlePostSubmit = async () => {
    if ((!content.trim() && !imageUrl) || isPosting) return;
    if (!session?.user?.email) {
      console.error('User not authenticated to create post.');
      return;
    }

    setIsPosting(true);
    try {
      const request = {
        content,
        imageUrl,
      };
      const response = await fetch(`/api/sosmed/posts`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
      });
      if (!response.ok) {
        throw new Error('Failed to create post');
      }
      setContent('');
      setImageUrl('');
      (document.querySelector('input[type="file"]') as HTMLInputElement).value = '';
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
    if (!session?.user?.email) {
      console.error('User not authenticated to like post.');
      return;
    }
    setIsLiking((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/likes`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error liking post:', error);
    } finally {
      setIsLiking((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleUnlike = async (postId: string) => {
    if (!session?.user?.email) {
      console.error('User not authenticated to unlike post.');
      return;
    }
    setIsLiking((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/likes`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ postId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error unliking post:', error);
    } finally {
      setIsLiking((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleAddComment = async (postId: string, comment: string) => {
    if (!comment?.trim()) return;
    if (!session?.user?.email) {
      console.error('User not authenticated to add comment.');
      return;
    }
    setIsCommenting((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content: comment, postId }),
      });
      mutate(`/api/sosmed/posts`);
      setNewComment(postId, '');
    } catch (error) {
      console.error('Error adding comment:', error);
    } finally {
      setIsCommenting((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleEditPost = async (postId: string, content: string) => {
    if (!session?.user?.email) {
      console.error('User not authenticated to edit post.');
      return;
    }
    setIsEditing((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: postId, content }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error editing post:', error);
    } finally {
      setIsEditing((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleDeletePost = async (postId: string) => {
    if (!session?.user?.email) {
      console.error('User not authenticated to delete post.');
      return;
    }
    setIsDeleting((prev) => ({ ...prev, [postId]: true }));
    try {
      await fetch(`/api/sosmed/posts`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: postId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error deleting post:', error);
    } finally {
      setIsDeleting((prev) => ({ ...prev, [postId]: false }));
    }
  };

  const handleEditComment = async (commentId: string, content: string) => {
    if (!session?.user?.email) {
      console.error('User not authenticated to edit comment.');
      return;
    }
    setIsEditing((prev) => ({ ...prev, [commentId]: true }));
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: commentId, content }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error editing comment:', error);
    } finally {
      setIsEditing((prev) => ({ ...prev, [commentId]: false }));
    }
  };

  const handleDeleteComment = async (commentId: string) => {
    if (!session?.user?.email) {
      console.error('User not authenticated to delete comment.');
      return;
    }
    setIsDeleting((prev) => ({ ...prev, [commentId]: true }));
    try {
      await fetch(`/api/sosmed/comments`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: commentId }),
      });
      mutate(`/api/sosmed/posts`);
    } catch (error) {
      console.error('Error deleting comment:', error);
    } finally {
      setIsDeleting((prev) => ({ ...prev, [commentId]: false }));
    }
  };

  return (
    <div className='container mx-auto py-8 px-4 max-w-4xl md:py-12 bg-background text-foreground'>
      <h1 className='text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent'>
        Community Hub
      </h1>

      {session?.user ? (
        <div className='mb-8 md:mb-10 group'>
          <div className='relative bg-gradient-to-br from-white/50 to-blue-50/50 dark:from-gray-800/50 dark:to-gray-900/50 rounded-2xl p-1 shadow-lg transition-all hover:shadow-xl'>
            <Card>
              <div className='space-y-4'>
                <Textarea
                  placeholder='Share your thoughts...'
                  value={content}
                  onChange={handleContentChange}
                  className='min-h-[120px] text-lg border-2 border-blue-200 focus:border-blue-400 dark:border-gray-700 dark:focus:border-blue-500 dark:bg-gray-800 rounded-xl transition-all'
                />
                <div className='flex flex-col gap-4'>
                  <label className='flex flex-col items-center justify-center gap-2 p-4 border-2 border-dashed border-blue-100 dark:border-gray-700 rounded-xl cursor-pointer hover:bg-blue-50/50 dark:hover:bg-gray-800/50 transition-colors'>
                    <UploadCloud className='w-8 h-8 text-blue-500' />
                    <span className='text-gray-600 dark:text-gray-400'>
                      {imageUrl ? 'Image attached' : 'Attach image or video'}
                    </span>
                    <input
                      type='file'
                      onChange={handleFileChange}
                      className='hidden'
                      id='file-input'
                    />
                  </label>
                  <ButtonA
                    onClick={handlePostSubmit}
                    disabled={isUploading || isPosting || (!content.trim() && !imageUrl)}
                    className='w-full py-3.5 px-6 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold rounded-xl shadow-md transition-all hover:shadow-lg hover:scale-[1.02] disabled:opacity-70 disabled:pointer-events-none'
                  >
                    {isUploading || isPosting ? (
                      <div className='flex items-center justify-center gap-2'>
                        <Loader2 className='w-5 h-5 animate-spin' aria-hidden="true" />
                        <span>{isPosting ? 'Publishing...' : 'Uploading...'}</span>
                      </div>
                    ) : (
                      'Publish Post'
                    )}
                  </ButtonA>
                </div>
              </div>
            </Card>
          </div>
        </div>
      ) : (
        <div className='mb-10 bg-gradient-to-br from-blue-50 to-purple-50 dark:from-gray-800 dark:to-gray-900/50 rounded-2xl p-6 text-center space-y-4'>
          <div className='flex flex-col items-center gap-4'>
            <Lock className='w-10 h-10 text-blue-500' />
            <h3 className='text-xl font-semibold text-gray-800 dark:text-gray-100'>
              Join the Conversation
            </h3>
            <p className='text-gray-600 dark:text-gray-400 mb-4'>
              Sign in to share your thoughts and engage with the community
            </p>
            <div className='w-full max-w-[150px] sm:max-w-[200px]'>
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
          (post: Posts & {
            user: ClientUser;
            likes: Likes[];
            comments: Comments[];
          }) => (
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
                  post.comments?.map((comment: Comments & { user?: ClientUser }) => ({
                    ...comment,
                    user: comment.user || {
                      id: '',
                      name: null,
                      email: null,
                      image: null,
                      emailVerified: null,
                      role: '',
                    },
                  })) || [],
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
              isEditing={isEditing}
              isDeleting={isDeleting}
            />
          )
        )}
      </div>
    </div>
  );
}
