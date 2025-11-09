import { ClientUser } from './ClientUser';
import { Post, Like, Comment } from '@asepharyana/services';

// Re-export with extended types if needed
export type { Post, Like, Comment };

// Extended post type with populated relations
export interface PostWithRelations extends Post {
  user: ClientUser;
  likes: Like[];
  comments: (Comment & { user: ClientUser })[];
}
