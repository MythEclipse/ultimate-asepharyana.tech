import { ClientUser } from './ClientUser';

export interface Posts {
  id: string;
  content: string;
  userId: string;
  postId: string;
  created_at: Date;
  updated_at: Date;
  authorId: string;
  image_url: string | null;
  user: ClientUser;
  likes: Likes[];
  comments: Comments[];
}

export interface Likes {
  id: string;
  userId: string;
  postId: string;
  created_at: Date;
}

export interface Comments {
  id: string;
  content: string;
  userId: string;
  postId: string;
  created_at: Date;
  updated_at: Date;
  authorId: string;
  user: ClientUser;
}
