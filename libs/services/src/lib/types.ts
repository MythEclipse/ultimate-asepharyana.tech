export interface Account {
  id: string;
  userId: string;
  type: string;
  provider: string;
  providerAccountId: string;
  refresh_token: string | null;
  access_token: string | null;
  expires_at: number | null;
  token_type: string | null;
  scope: string | null;
  id_token: string | null;
  session_state: string | null;
}

export interface Session {
  id: string;
  sessionToken: string;
  userId: string;
  expires: Date;
}

export interface User {
  id?: string;
  name: string | null;
  email: string | null;
  emailVerified: Date | null;
  image: string | null;
  password: string | null;
  refreshToken: string | null;
  role: string;
}

export interface Role {
  id: string;
  name: string;
  description: string | null;
}

export interface Permission {
  id: string;
  name: string;
  description: string | null;
}

export interface UserRole {
  userId: string;
  roleId: string;
}

export interface RolePermission {
  roleId: string;
  permissionId: string;
}

export interface Comments {
  id?: string;
  userId: string;
  postId: string;
  authorId: string;
  content: string;
  created_at?: Date;
  updated_at?: Date;
}

export interface Likes {
  userId: string;
  postId: string;
}

export interface Posts {
  id?: string;
  userId: string;
  content: string;
  image_url: string | null;
  created_at?: Date;
  updated_at?: Date;
  authorId: string;
}

export interface Replies {
  id?: string;
  userId: string;
  commentId: string;
  content: string;
  created_at?: Date;
}

export interface ChatMessage {
  id?: string;
  userId: string;
  text: string;
  email: string | null;
  imageProfile: string | null;
  imageMessage: string | null;
  role: string | null;
  timestamp?: Date;
}

export interface DB {
  Account: Account;
  Session: Session;
  User: User;
  Role: Role;
  Permission: Permission;
  UserRole: UserRole;
  RolePermission: RolePermission;
  Comments: Comments;
  Likes: Likes;
  Posts: Posts;
  Replies: Replies;
  ChatMessage: ChatMessage;
}
