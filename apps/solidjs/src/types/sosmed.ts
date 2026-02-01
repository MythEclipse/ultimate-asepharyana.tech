export interface User {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
}

export interface Comment {
  id: string;
  postId: string;
  userId: string;
  authorId: string;
  content: string;
  created_at: string;
  updated_at: string;
  user: User;
}

export interface Like {
  userId: string;
  postId: string;
  user: {
    id: string;
    name: string | null;
    email: string | null;
  };
}

export interface Post {
  id: string;
  userId: string;
  authorId: string;
  content: string;
  image_url: string | null;
  created_at: string;
  updated_at: string;
  user: User;
  comments: Comment[];
  likes: Like[];
}
