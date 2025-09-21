import { HttpClient } from '../utils/httpClient';
import logger from '../utils/logger';

export class SosmedService {
  static async getPosts() {
    try {
      return await HttpClient.fetchJson('/api/sosmed/posts');
    } catch (error) {
      logger.error('Failed to fetch posts', { error });
      throw error;
    }
  }

  static async createPost(data: { content: string; imageUrl?: string }) {
    try {
      return await HttpClient.request('/api/sosmed/posts', 'POST', data);
    } catch (error) {
      logger.error('Failed to create post', { error, data });
      throw error;
    }
  }

  static async updatePost(id: string, data: { content: string }) {
    try {
      return await HttpClient.request('/api/sosmed/posts', 'PUT', { id, ...data });
    } catch (error) {
      logger.error('Failed to update post', { error, id, data });
      throw error;
    }
  }

  static async deletePost(id: string) {
    try {
      return await HttpClient.request('/api/sosmed/posts', 'DELETE', { id });
    } catch (error) {
      logger.error('Failed to delete post', { error, id });
      throw error;
    }
  }

  static async likePost(postId: string) {
    try {
      return await HttpClient.request('/api/sosmed/likes', 'POST', { postId });
    } catch (error) {
      logger.error('Failed to like post', { error, postId });
      throw error;
    }
  }

  static async unlikePost(postId: string) {
    try {
      return await HttpClient.request('/api/sosmed/likes', 'DELETE', { postId });
    } catch (error) {
      logger.error('Failed to unlike post', { error, postId });
      throw error;
    }
  }

  static async addComment(data: { content: string; postId: string }) {
    try {
      return await HttpClient.request('/api/sosmed/comments', 'POST', data);
    } catch (error) {
      logger.error('Failed to add comment', { error, data });
      throw error;
    }
  }

  static async updateComment(id: string, data: { content: string }) {
    try {
      return await HttpClient.request('/api/sosmed/comments', 'PUT', { id, ...data });
    } catch (error) {
      logger.error('Failed to update comment', { error, id, data });
      throw error;
    }
  }

  static async deleteComment(id: string) {
    try {
      return await HttpClient.request('/api/sosmed/comments', 'DELETE', { id });
    } catch (error) {
      logger.error('Failed to delete comment', { error, id });
      throw error;
    }
  }
}
