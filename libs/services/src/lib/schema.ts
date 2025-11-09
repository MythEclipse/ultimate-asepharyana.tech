import {
  mysqlTable,
  varchar,
  timestamp,
  int,
  text,
  primaryKey,
  index,
} from 'drizzle-orm/mysql-core';
import { relations } from 'drizzle-orm';

// User table
export const users = mysqlTable(
  'User',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    name: varchar('name', { length: 255 }),
    email: varchar('email', { length: 255 }),
    emailVerified: timestamp('emailVerified'),
    image: text('image'),
    password: varchar('password', { length: 255 }),
    refreshToken: text('refreshToken'),
    role: varchar('role', { length: 50 }).notNull().default('user'),
  },
  (table) => ({
    emailIdx: index('email_idx').on(table.email),
  })
);

// Account table
export const accounts = mysqlTable(
  'Account',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    type: varchar('type', { length: 255 }).notNull(),
    provider: varchar('provider', { length: 255 }).notNull(),
    providerAccountId: varchar('providerAccountId', { length: 255 }).notNull(),
    refresh_token: text('refresh_token'),
    access_token: text('access_token'),
    expires_at: int('expires_at'),
    token_type: varchar('token_type', { length: 255 }),
    scope: varchar('scope', { length: 255 }),
    id_token: text('id_token'),
    session_state: varchar('session_state', { length: 255 }),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
  })
);

// Session table
export const sessions = mysqlTable(
  'Session',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    sessionToken: varchar('sessionToken', { length: 255 }).notNull().unique(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    expires: timestamp('expires').notNull(),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    sessionTokenIdx: index('sessionToken_idx').on(table.sessionToken),
  })
);

// Role table
export const roles = mysqlTable('Role', {
  id: varchar('id', { length: 255 }).primaryKey(),
  name: varchar('name', { length: 255 }).notNull().unique(),
  description: text('description'),
});

// Permission table
export const permissions = mysqlTable('Permission', {
  id: varchar('id', { length: 255 }).primaryKey(),
  name: varchar('name', { length: 255 }).notNull().unique(),
  description: text('description'),
});

// UserRole junction table
export const userRoles = mysqlTable(
  'UserRole',
  {
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    roleId: varchar('roleId', { length: 255 })
      .notNull()
      .references(() => roles.id, { onDelete: 'cascade' }),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.userId, table.roleId] }),
  })
);

// RolePermission junction table
export const rolePermissions = mysqlTable(
  'RolePermission',
  {
    roleId: varchar('roleId', { length: 255 })
      .notNull()
      .references(() => roles.id, { onDelete: 'cascade' }),
    permissionId: varchar('permissionId', { length: 255 })
      .notNull()
      .references(() => permissions.id, { onDelete: 'cascade' }),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.roleId, table.permissionId] }),
  })
);

// Posts table
export const posts = mysqlTable(
  'Posts',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    authorId: varchar('authorId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    content: text('content').notNull(),
    image_url: text('image_url'),
    created_at: timestamp('created_at').notNull().defaultNow(),
    updated_at: timestamp('updated_at').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    authorIdIdx: index('authorId_idx').on(table.authorId),
    createdAtIdx: index('created_at_idx').on(table.created_at),
  })
);

// Comments table
export const comments = mysqlTable(
  'Comments',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    postId: varchar('postId', { length: 255 })
      .notNull()
      .references(() => posts.id, { onDelete: 'cascade' }),
    authorId: varchar('authorId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    content: text('content').notNull(),
    created_at: timestamp('created_at').notNull().defaultNow(),
    updated_at: timestamp('updated_at').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    postIdIdx: index('postId_idx').on(table.postId),
    userIdIdx: index('userId_idx').on(table.userId),
    authorIdIdx: index('authorId_idx').on(table.authorId),
  })
);

// Likes table
export const likes = mysqlTable(
  'Likes',
  {
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    postId: varchar('postId', { length: 255 })
      .notNull()
      .references(() => posts.id, { onDelete: 'cascade' }),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.userId, table.postId] }),
  })
);

// Replies table
export const replies = mysqlTable(
  'Replies',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    commentId: varchar('commentId', { length: 255 })
      .notNull()
      .references(() => comments.id, { onDelete: 'cascade' }),
    content: text('content').notNull(),
    created_at: timestamp('created_at').notNull().defaultNow(),
  },
  (table) => ({
    commentIdIdx: index('commentId_idx').on(table.commentId),
    userIdIdx: index('userId_idx').on(table.userId),
  })
);

// ChatMessage table
export const chatMessages = mysqlTable(
  'ChatMessage',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    text: text('text').notNull(),
    email: varchar('email', { length: 255 }),
    imageProfile: text('imageProfile'),
    imageMessage: text('imageMessage'),
    role: varchar('role', { length: 50 }),
    timestamp: timestamp('timestamp').notNull().defaultNow(),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    timestampIdx: index('timestamp_idx').on(table.timestamp),
  })
);

// Relations
export const usersRelations = relations(users, ({ many }) => ({
  accounts: many(accounts),
  sessions: many(sessions),
  posts: many(posts),
  comments: many(comments),
  likes: many(likes),
  replies: many(replies),
  chatMessages: many(chatMessages),
  userRoles: many(userRoles),
}));

export const accountsRelations = relations(accounts, ({ one }) => ({
  user: one(users, {
    fields: [accounts.userId],
    references: [users.id],
  }),
}));

export const sessionsRelations = relations(sessions, ({ one }) => ({
  user: one(users, {
    fields: [sessions.userId],
    references: [users.id],
  }),
}));

export const postsRelations = relations(posts, ({ one, many }) => ({
  user: one(users, {
    fields: [posts.userId],
    references: [users.id],
  }),
  author: one(users, {
    fields: [posts.authorId],
    references: [users.id],
  }),
  comments: many(comments),
  likes: many(likes),
}));

export const commentsRelations = relations(comments, ({ one, many }) => ({
  user: one(users, {
    fields: [comments.userId],
    references: [users.id],
  }),
  post: one(posts, {
    fields: [comments.postId],
    references: [posts.id],
  }),
  author: one(users, {
    fields: [comments.authorId],
    references: [users.id],
  }),
  replies: many(replies),
}));

export const likesRelations = relations(likes, ({ one }) => ({
  user: one(users, {
    fields: [likes.userId],
    references: [users.id],
  }),
  post: one(posts, {
    fields: [likes.postId],
    references: [posts.id],
  }),
}));

export const repliesRelations = relations(replies, ({ one }) => ({
  user: one(users, {
    fields: [replies.userId],
    references: [users.id],
  }),
  comment: one(comments, {
    fields: [replies.commentId],
    references: [comments.id],
  }),
}));

export const chatMessagesRelations = relations(chatMessages, ({ one }) => ({
  user: one(users, {
    fields: [chatMessages.userId],
    references: [users.id],
  }),
}));

export const rolesRelations = relations(roles, ({ many }) => ({
  userRoles: many(userRoles),
  rolePermissions: many(rolePermissions),
}));

export const permissionsRelations = relations(permissions, ({ many }) => ({
  rolePermissions: many(rolePermissions),
}));

export const userRolesRelations = relations(userRoles, ({ one }) => ({
  user: one(users, {
    fields: [userRoles.userId],
    references: [users.id],
  }),
  role: one(roles, {
    fields: [userRoles.roleId],
    references: [roles.id],
  }),
}));

export const rolePermissionsRelations = relations(
  rolePermissions,
  ({ one }) => ({
    role: one(roles, {
      fields: [rolePermissions.roleId],
      references: [roles.id],
    }),
    permission: one(permissions, {
      fields: [rolePermissions.permissionId],
      references: [permissions.id],
    }),
  })
);
