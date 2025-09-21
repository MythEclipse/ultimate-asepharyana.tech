import { useState, useEffect } from 'react';
import axios from 'axios';
import { BaseUrl } from '../../lib/url';
import { Posts } from '@asepharyana/services';
import { buildUrlWithParams } from '../url-utils';

export const usePosts = () => {
  const [posts, setPosts] = useState<Posts[]>([]);

  const fetchPosts = async () => {
    try {
      // Use centralized URL builder for better consistency
      const url = buildUrlWithParams({
        baseUrl: BaseUrl,
        path: '/api/sosmed/posts',
      });

      const { data } = await axios.get(url);
      setPosts(data.posts);
    } catch (error) {
      console.error('Error fetching posts:', error);
    }
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  return { posts, fetchPosts };
};
