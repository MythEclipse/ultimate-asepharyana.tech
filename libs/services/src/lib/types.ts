import type { InferSelectModel, InferInsertModel } from 'drizzle-orm';
import * as schema from './schema';

// Export schema for use in applications
export * from './schema';

// Infer types from schema
export type User = InferSelectModel<typeof schema.users>;
export type NewUser = InferInsertModel<typeof schema.users>;

export type Account = InferSelectModel<typeof schema.accounts>;
export type NewAccount = InferInsertModel<typeof schema.accounts>;

export type Session = InferSelectModel<typeof schema.sessions>;
export type NewSession = InferInsertModel<typeof schema.sessions>;

export type Role = InferSelectModel<typeof schema.roles>;
export type NewRole = InferInsertModel<typeof schema.roles>;

export type Permission = InferSelectModel<typeof schema.permissions>;
export type NewPermission = InferInsertModel<typeof schema.permissions>;

export type UserRole = InferSelectModel<typeof schema.userRoles>;
export type NewUserRole = InferInsertModel<typeof schema.userRoles>;

export type RolePermission = InferSelectModel<typeof schema.rolePermissions>;
export type NewRolePermission = InferInsertModel<typeof schema.rolePermissions>;

export type Post = InferSelectModel<typeof schema.posts>;
export type NewPost = InferInsertModel<typeof schema.posts>;

export type Comment = InferSelectModel<typeof schema.comments>;
export type NewComment = InferInsertModel<typeof schema.comments>;

export type Like = InferSelectModel<typeof schema.likes>;
export type NewLike = InferInsertModel<typeof schema.likes>;

export type Reply = InferSelectModel<typeof schema.replies>;
export type NewReply = InferInsertModel<typeof schema.replies>;

export type ChatMessage = InferSelectModel<typeof schema.chatMessages>;
export type NewChatMessage = InferInsertModel<typeof schema.chatMessages>;

// Legacy compatibility exports (mapped to new types)
export type { User as Users };
export type { Post as Posts };
export type { Comment as Comments };
export type { Like as Likes };
export type { Reply as Replies };

// Database type for backward compatibility
export interface DB {
  Account: Account;
  Session: Session;
  User: User;
  Role: Role;
  Permission: Permission;
  UserRole: UserRole;
  RolePermission: RolePermission;
  Comments: Comment;
  Likes: Like;
  Posts: Post;
  Replies: Reply;
  ChatMessage: ChatMessage;
}
