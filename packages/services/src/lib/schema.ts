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
  }),
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
  }),
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
  }),
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
  }),
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
  }),
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
  }),
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
  }),
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
  }),
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
  }),
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
  }),
);

// EmailVerificationToken table
export const emailVerificationTokens = mysqlTable(
  'EmailVerificationToken',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    token: varchar('token', { length: 255 }).notNull().unique(),
    expiresAt: timestamp('expiresAt').notNull(),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    tokenIdx: index('token_idx').on(table.token),
  }),
);

// PasswordResetToken table
export const passwordResetTokens = mysqlTable(
  'PasswordResetToken',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    token: varchar('token', { length: 255 }).notNull().unique(),
    expiresAt: timestamp('expiresAt').notNull(),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    used: int('used').notNull().default(0),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    tokenIdx: index('token_idx').on(table.token),
  }),
);

// ChatRoom table
export const chatRooms = mysqlTable(
  'ChatRoom',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    name: varchar('name', { length: 255 }).notNull(),
    description: text('description'),
    isPrivate: int('isPrivate').notNull().default(0),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    updatedAt: timestamp('updatedAt').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    createdAtIdx: index('createdAt_idx').on(table.createdAt),
  }),
);

// Update ChatMessage table to reference ChatRoom
export const chatMessagesWithRoom = mysqlTable(
  'ChatMessage_room',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    roomId: varchar('roomId', { length: 255 })
      .notNull()
      .references(() => chatRooms.id, { onDelete: 'cascade' }),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    content: text('content').notNull(),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    updatedAt: timestamp('updatedAt').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    roomIdIdx: index('roomId_idx').on(table.roomId),
    userIdIdx: index('userId_idx').on(table.userId),
    createdAtIdx: index('createdAt_idx').on(table.createdAt),
  }),
);

// ChatRoomMember table
export const chatRoomMembers = mysqlTable(
  'ChatRoomMember',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    roomId: varchar('roomId', { length: 255 })
      .notNull()
      .references(() => chatRooms.id, { onDelete: 'cascade' }),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    role: varchar('role', { length: 50 }).notNull().default('member'),
    joinedAt: timestamp('joinedAt').notNull().defaultNow(),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.roomId, table.userId] }),
    roomIdIdx: index('roomId_idx').on(table.roomId),
    userIdIdx: index('userId_idx').on(table.userId),
  }),
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
  emailVerificationTokens: many(emailVerificationTokens),
  passwordResetTokens: many(passwordResetTokens),
  chatRoomMembers: many(chatRoomMembers),
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
  }),
);

export const emailVerificationTokensRelations = relations(
  emailVerificationTokens,
  ({ one }) => ({
    user: one(users, {
      fields: [emailVerificationTokens.userId],
      references: [users.id],
    }),
  }),
);

export const passwordResetTokensRelations = relations(
  passwordResetTokens,
  ({ one }) => ({
    user: one(users, {
      fields: [passwordResetTokens.userId],
      references: [users.id],
    }),
  }),
);

export const chatRoomsRelations = relations(chatRooms, ({ many }) => ({
  messages: many(chatMessagesWithRoom),
  members: many(chatRoomMembers),
}));

export const chatMessagesWithRoomRelations = relations(
  chatMessagesWithRoom,
  ({ one }) => ({
    room: one(chatRooms, {
      fields: [chatMessagesWithRoom.roomId],
      references: [chatRooms.id],
    }),
    user: one(users, {
      fields: [chatMessagesWithRoom.userId],
      references: [users.id],
    }),
  }),
);

export const chatRoomMembersRelations = relations(
  chatRoomMembers,
  ({ one }) => ({
    room: one(chatRooms, {
      fields: [chatRoomMembers.roomId],
      references: [chatRooms.id],
    }),
    user: one(users, {
      fields: [chatRoomMembers.userId],
      references: [users.id],
    }),
  }),
);

// =============================================
// QUIZ BATTLE SCHEMA
// =============================================

// QuizQuestion table - Bank soal
export const quizQuestions = mysqlTable(
  'QuizQuestion',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    text: text('text').notNull(),
    category: varchar('category', { length: 100 }).notNull(),
    difficulty: varchar('difficulty', { length: 50 }).notNull(), // easy, medium, hard
    correctAnswer: int('correctAnswer').notNull(), // index 0-3
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    updatedAt: timestamp('updatedAt').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    categoryIdx: index('category_idx').on(table.category),
    difficultyIdx: index('difficulty_idx').on(table.difficulty),
  }),
);

// QuizAnswer table - Pilihan jawaban
export const quizAnswers = mysqlTable(
  'QuizAnswer',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    questionId: varchar('questionId', { length: 255 })
      .notNull()
      .references(() => quizQuestions.id, { onDelete: 'cascade' }),
    text: text('text').notNull(),
    answerIndex: int('answerIndex').notNull(), // 0-3
  },
  (table) => ({
    questionIdIdx: index('questionId_idx').on(table.questionId),
  }),
);

// QuizUserStats table - Statistik user
export const quizUserStats = mysqlTable(
  'QuizUserStats',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' })
      .unique(),
    points: int('points').notNull().default(0),
    wins: int('wins').notNull().default(0),
    losses: int('losses').notNull().default(0),
    draws: int('draws').notNull().default(0),
    totalGames: int('totalGames').notNull().default(0),
    currentStreak: int('currentStreak').notNull().default(0),
    bestStreak: int('bestStreak').notNull().default(0),
    totalCorrectAnswers: int('totalCorrectAnswers').notNull().default(0),
    totalQuestions: int('totalQuestions').notNull().default(0),
    level: int('level').notNull().default(1),
    experience: int('experience').notNull().default(0),
    coins: int('coins').notNull().default(0),
    updatedAt: timestamp('updatedAt').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    pointsIdx: index('points_idx').on(table.points),
  }),
);

// QuizMatch table - Data match/pertandingan
export const quizMatches = mysqlTable(
  'QuizMatch',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    player1Id: varchar('player1Id', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    player2Id: varchar('player2Id', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    winnerId: varchar('winnerId', { length: 255 }).references(() => users.id),
    gameMode: varchar('gameMode', { length: 50 }).notNull(), // ranked, casual, friend
    difficulty: varchar('difficulty', { length: 50 }).notNull(),
    category: varchar('category', { length: 100 }).notNull(),
    status: varchar('status', { length: 50 }).notNull().default('waiting'), // waiting, playing, finished, cancelled
    player1Score: int('player1Score').notNull().default(0),
    player2Score: int('player2Score').notNull().default(0),
    player1Health: int('player1Health').notNull().default(100),
    player2Health: int('player2Health').notNull().default(100),
    currentQuestionIndex: int('currentQuestionIndex').notNull().default(0),
    totalQuestions: int('totalQuestions').notNull().default(5),
    timePerQuestion: int('timePerQuestion').notNull().default(10),
    startedAt: timestamp('startedAt'),
    finishedAt: timestamp('finishedAt'),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
  },
  (table) => ({
    player1IdIdx: index('player1Id_idx').on(table.player1Id),
    player2IdIdx: index('player2Id_idx').on(table.player2Id),
    statusIdx: index('status_idx').on(table.status),
    createdAtIdx: index('createdAt_idx').on(table.createdAt),
  }),
);

// QuizMatchQuestion table - Pertanyaan yang digunakan dalam match
export const quizMatchQuestions = mysqlTable(
  'QuizMatchQuestion',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    matchId: varchar('matchId', { length: 255 })
      .notNull()
      .references(() => quizMatches.id, { onDelete: 'cascade' }),
    questionId: varchar('questionId', { length: 255 })
      .notNull()
      .references(() => quizQuestions.id, { onDelete: 'cascade' }),
    questionIndex: int('questionIndex').notNull(),
  },
  (table) => ({
    matchIdIdx: index('matchId_idx').on(table.matchId),
  }),
);

// QuizMatchAnswer table - Jawaban player dalam match
export const quizMatchAnswers = mysqlTable(
  'QuizMatchAnswer',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    matchId: varchar('matchId', { length: 255 })
      .notNull()
      .references(() => quizMatches.id, { onDelete: 'cascade' }),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    questionId: varchar('questionId', { length: 255 })
      .notNull()
      .references(() => quizQuestions.id, { onDelete: 'cascade' }),
    questionIndex: int('questionIndex').notNull(),
    answerIndex: int('answerIndex').notNull(),
    isCorrect: int('isCorrect').notNull(),
    answerTime: int('answerTime').notNull(), // milliseconds
    points: int('points').notNull().default(0),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
  },
  (table) => ({
    matchIdIdx: index('matchId_idx').on(table.matchId),
    userIdIdx: index('userId_idx').on(table.userId),
  }),
);

// QuizFriendship table - Sistem pertemanan
export const quizFriendships = mysqlTable(
  'QuizFriendship',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    friendId: varchar('friendId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    status: varchar('status', { length: 50 }).notNull().default('pending'), // pending, accepted, rejected
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    updatedAt: timestamp('updatedAt').notNull().defaultNow().onUpdateNow(),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.userId, table.friendId] }),
    userIdIdx: index('userId_idx').on(table.userId),
    friendIdIdx: index('friendId_idx').on(table.friendId),
    statusIdx: index('status_idx').on(table.status),
  }),
);

// QuizLobby table - Lobby untuk private match
export const quizLobbies = mysqlTable(
  'QuizLobby',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    lobbyCode: varchar('lobbyCode', { length: 8 }).notNull().unique(),
    hostId: varchar('hostId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    isPrivate: int('isPrivate').notNull().default(1),
    maxPlayers: int('maxPlayers').notNull().default(2),
    status: varchar('status', { length: 50 }).notNull().default('waiting'), // waiting, starting, playing, finished
    difficulty: varchar('difficulty', { length: 50 }).notNull(),
    category: varchar('category', { length: 100 }).notNull(),
    totalQuestions: int('totalQuestions').notNull().default(5),
    timePerQuestion: int('timePerQuestion').notNull().default(10),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    expiresAt: timestamp('expiresAt').notNull(),
  },
  (table) => ({
    lobbyCodeIdx: index('lobbyCode_idx').on(table.lobbyCode),
    hostIdIdx: index('hostId_idx').on(table.hostId),
    statusIdx: index('status_idx').on(table.status),
  }),
);

// QuizLobbyMember table - Member dalam lobby
export const quizLobbyMembers = mysqlTable(
  'QuizLobbyMember',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    lobbyId: varchar('lobbyId', { length: 255 })
      .notNull()
      .references(() => quizLobbies.id, { onDelete: 'cascade' }),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    isHost: int('isHost').notNull().default(0),
    isReady: int('isReady').notNull().default(0),
    joinedAt: timestamp('joinedAt').notNull().defaultNow(),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.lobbyId, table.userId] }),
    lobbyIdIdx: index('lobbyId_idx').on(table.lobbyId),
    userIdIdx: index('userId_idx').on(table.userId),
  }),
);

// QuizNotification table - Sistem notifikasi
export const quizNotifications = mysqlTable(
  'QuizNotification',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    type: varchar('type', { length: 50 }).notNull(), // achievement, friend_request, challenge, system
    title: varchar('title', { length: 255 }).notNull(),
    message: text('message').notNull(),
    data: text('data'), // JSON data
    isRead: int('isRead').notNull().default(0),
    priority: varchar('priority', { length: 50 }).notNull().default('medium'), // low, medium, high
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    expiresAt: timestamp('expiresAt'),
  },
  (table) => ({
    userIdIdx: index('userId_idx').on(table.userId),
    isReadIdx: index('isRead_idx').on(table.isRead),
    createdAtIdx: index('createdAt_idx').on(table.createdAt),
  }),
);

// QuizAchievement table - Data achievement
export const quizAchievements = mysqlTable(
  'QuizAchievement',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    name: varchar('name', { length: 255 }).notNull(),
    description: text('description').notNull(),
    icon: varchar('icon', { length: 100 }),
    rarity: varchar('rarity', { length: 50 }).notNull().default('common'), // common, rare, epic, legendary
    requirement: text('requirement').notNull(), // JSON
    rewardPoints: int('rewardPoints').notNull().default(0),
    rewardCoins: int('rewardCoins').notNull().default(0),
  },
  (table) => ({
    rarityIdx: index('rarity_idx').on(table.rarity),
  }),
);

// QuizUserAchievement table - Achievement yang dimiliki user
export const quizUserAchievements = mysqlTable(
  'QuizUserAchievement',
  {
    id: varchar('id', { length: 255 }).primaryKey(),
    userId: varchar('userId', { length: 255 })
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    achievementId: varchar('achievementId', { length: 255 })
      .notNull()
      .references(() => quizAchievements.id, { onDelete: 'cascade' }),
    unlockedAt: timestamp('unlockedAt').notNull().defaultNow(),
  },
  (table) => ({
    pk: primaryKey({ columns: [table.userId, table.achievementId] }),
    userIdIdx: index('userId_idx').on(table.userId),
  }),
);

// Quiz Battle Relations
export const quizQuestionsRelations = relations(quizQuestions, ({ many }) => ({
  answers: many(quizAnswers),
  matchQuestions: many(quizMatchQuestions),
  matchAnswers: many(quizMatchAnswers),
}));

export const quizAnswersRelations = relations(quizAnswers, ({ one }) => ({
  question: one(quizQuestions, {
    fields: [quizAnswers.questionId],
    references: [quizQuestions.id],
  }),
}));

export const quizUserStatsRelations = relations(quizUserStats, ({ one }) => ({
  user: one(users, {
    fields: [quizUserStats.userId],
    references: [users.id],
  }),
}));

export const quizMatchesRelations = relations(quizMatches, ({ one, many }) => ({
  player1: one(users, {
    fields: [quizMatches.player1Id],
    references: [users.id],
  }),
  player2: one(users, {
    fields: [quizMatches.player2Id],
    references: [users.id],
  }),
  winner: one(users, {
    fields: [quizMatches.winnerId],
    references: [users.id],
  }),
  matchQuestions: many(quizMatchQuestions),
  matchAnswers: many(quizMatchAnswers),
}));

export const quizMatchQuestionsRelations = relations(
  quizMatchQuestions,
  ({ one }) => ({
    match: one(quizMatches, {
      fields: [quizMatchQuestions.matchId],
      references: [quizMatches.id],
    }),
    question: one(quizQuestions, {
      fields: [quizMatchQuestions.questionId],
      references: [quizQuestions.id],
    }),
  }),
);

export const quizMatchAnswersRelations = relations(
  quizMatchAnswers,
  ({ one }) => ({
    match: one(quizMatches, {
      fields: [quizMatchAnswers.matchId],
      references: [quizMatches.id],
    }),
    user: one(users, {
      fields: [quizMatchAnswers.userId],
      references: [users.id],
    }),
    question: one(quizQuestions, {
      fields: [quizMatchAnswers.questionId],
      references: [quizQuestions.id],
    }),
  }),
);

export const quizFriendshipsRelations = relations(
  quizFriendships,
  ({ one }) => ({
    user: one(users, {
      fields: [quizFriendships.userId],
      references: [users.id],
    }),
    friend: one(users, {
      fields: [quizFriendships.friendId],
      references: [users.id],
    }),
  }),
);

export const quizLobbiesRelations = relations(quizLobbies, ({ one, many }) => ({
  host: one(users, {
    fields: [quizLobbies.hostId],
    references: [users.id],
  }),
  members: many(quizLobbyMembers),
}));

export const quizLobbyMembersRelations = relations(
  quizLobbyMembers,
  ({ one }) => ({
    lobby: one(quizLobbies, {
      fields: [quizLobbyMembers.lobbyId],
      references: [quizLobbies.id],
    }),
    user: one(users, {
      fields: [quizLobbyMembers.userId],
      references: [users.id],
    }),
  }),
);

export const quizNotificationsRelations = relations(
  quizNotifications,
  ({ one }) => ({
    user: one(users, {
      fields: [quizNotifications.userId],
      references: [users.id],
    }),
  }),
);

export const quizAchievementsRelations = relations(
  quizAchievements,
  ({ many }) => ({
    userAchievements: many(quizUserAchievements),
  }),
);

export const quizUserAchievementsRelations = relations(
  quizUserAchievements,
  ({ one }) => ({
    user: one(users, {
      fields: [quizUserAchievements.userId],
      references: [users.id],
    }),
    achievement: one(quizAchievements, {
      fields: [quizUserAchievements.achievementId],
      references: [quizAchievements.id],
    }),
  }),
);

// =============================================
// IMAGE CACHE SCHEMA
// =============================================

// ImageCache table - URL mappings for CDN caching
export const imageCache = mysqlTable(
  'ImageCache',
  {
    id: varchar('id', { length: 36 }).primaryKey(),
    originalUrl: text('originalUrl').notNull(),
    cdnUrl: text('cdnUrl').notNull(),
    createdAt: timestamp('createdAt').notNull().defaultNow(),
    expiresAt: timestamp('expiresAt'),
  },
  (table) => ({
    originalUrlIdx: index('originalUrl_idx').on(table.originalUrl),
  }),
);
