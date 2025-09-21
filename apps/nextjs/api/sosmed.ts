import { HttpClient } from '../utils/httpClient';
import logger from '../utils/logger';
import { toAppError, logError } from '../utils/error-handler';

export class SosmedService {
  static async getPosts() {
    try {
      return await HttpClient.fetchJson('/api/sosmed/posts');
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/posts', method: 'GET' });
      logError(appError);
      logger.error('Failed to fetch posts', appError);
      throw appError;
    }
  }

  static async createPost(data: { content: string; imageUrl?: string }) {
    try {
      return await HttpClient.request('/api/sosmed/posts', 'POST', data);
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/posts', method: 'POST', context: data });
      logError(appError);
      logger.error('Failed to create post', { error: appError, data });
      throw appError;
    }
  }

  static async updatePost(id: string, data: { content: string }) {
    try {
      return await HttpClient.request('/api/sosmed/posts', 'PUT', { id, ...data });
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/posts', method: 'PUT', context: { id, ...data } });
      logError(appError);
      logger.error('Failed to update post', { error: appError, id, data });
      throw appError;
    }
  }

  static async deletePost(id: string) {
    try {
      return await HttpClient.request('/api/sosmed/posts', 'DELETE', { id });
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/posts', method: 'DELETE', context: { id } });
      logError(appError);
      logger.error('Failed to delete post', { error: appError, id });
      throw appError;
    }
  }

  static async likePost(postId: string) {
    try {
      return await HttpClient.request('/api/sosmed/likes', 'POST', { postId });
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/likes', method: 'POST', context: { postId } });
      logError(appError);
      logger.error('Failed to like post', { error: appError, postId });
      throw appError;
    }
  }

  static async unlikePost(postId: string) {
    try {
      return await HttpClient.request('/api/sosmed/likes', 'DELETE', { postId });
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/likes', method: 'DELETE', context: { postId } });
      logError(appError);
      logger.error('Failed to unlike post', { error: appError, postId });
      throw appError;
    }
  }

  static async addComment(data: { content: string; postId: string }) {
    try {
      return await HttpClient.request('/api/sosmed/comments', 'POST', data);
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/comments', method: 'POST', context: data });
      logError(appError);
      logger.error('Failed to add comment', { error: appError, data });
      throw appError;
    }
  }

  static async updateComment(id: string, data: { content: string }) {
    try {
      return await HttpClient.request('/api/sosmed/comments', 'PUT', { id, ...data });
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/comments', method: 'PUT', context: { id, ...data } });
      logError(appError);
      logger.error('Failed to update comment', { error: appError, id, data });
      throw appError;
    }
  }

  static async deleteComment(id: string) {
    try {
      return await HttpClient.request('/api/sosmed/comments', 'DELETE', { id });
    } catch (error) {
      const appError = toAppError(error, { url: '/api/sosmed/comments', method: 'DELETE', context: { id } });
      logError(appError);
      logger.error('Failed to delete comment', { error: appError, id });
      throw appError;
    }
  }
}
