CREATE TABLE `Account` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`type` varchar(255) NOT NULL,
	`provider` varchar(255) NOT NULL,
	`providerAccountId` varchar(255) NOT NULL,
	`refresh_token` text,
	`access_token` text,
	`expires_at` int,
	`token_type` varchar(255),
	`scope` varchar(255),
	`id_token` text,
	`session_state` varchar(255),
	CONSTRAINT `Account_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `ChatMessage` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`text` text NOT NULL,
	`email` varchar(255),
	`imageProfile` text,
	`imageMessage` text,
	`role` varchar(50),
	`timestamp` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `ChatMessage_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `ChatMessage_room` (
	`id` varchar(255) NOT NULL,
	`roomId` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`content` text NOT NULL,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `ChatMessage_room_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `ChatRoomMember` (
	`id` varchar(255) NOT NULL,
	`roomId` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`role` varchar(50) NOT NULL DEFAULT 'member',
	`joinedAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `ChatRoomMember_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `ChatRoom` (
	`id` varchar(255) NOT NULL,
	`name` varchar(255) NOT NULL,
	`description` text,
	`isPrivate` int NOT NULL DEFAULT 0,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `ChatRoom_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `Comments` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`postId` varchar(255) NOT NULL,
	`authorId` varchar(255) NOT NULL,
	`content` text NOT NULL,
	`created_at` timestamp NOT NULL DEFAULT (now()),
	`updated_at` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `Comments_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `EmailVerificationToken` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`token` varchar(255) NOT NULL,
	`expiresAt` timestamp NOT NULL,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `EmailVerificationToken_id` PRIMARY KEY(`id`),
	CONSTRAINT `EmailVerificationToken_token_unique` UNIQUE(`token`)
);
--> statement-breakpoint
CREATE TABLE `ImageCache` (
	`id` varchar(36) NOT NULL,
	`originalUrl` text NOT NULL,
	`cdnUrl` text NOT NULL,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`expiresAt` timestamp,
	CONSTRAINT `ImageCache_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `Likes` (
	`userId` varchar(255) NOT NULL,
	`postId` varchar(255) NOT NULL,
	CONSTRAINT `Likes_userId_postId_pk` PRIMARY KEY(`userId`,`postId`)
);
--> statement-breakpoint
CREATE TABLE `PasswordResetToken` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`token` varchar(255) NOT NULL,
	`expiresAt` timestamp NOT NULL,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`used` int NOT NULL DEFAULT 0,
	CONSTRAINT `PasswordResetToken_id` PRIMARY KEY(`id`),
	CONSTRAINT `PasswordResetToken_token_unique` UNIQUE(`token`)
);
--> statement-breakpoint
CREATE TABLE `Permission` (
	`id` varchar(255) NOT NULL,
	`name` varchar(255) NOT NULL,
	`description` text,
	CONSTRAINT `Permission_id` PRIMARY KEY(`id`),
	CONSTRAINT `Permission_name_unique` UNIQUE(`name`)
);
--> statement-breakpoint
CREATE TABLE `Posts` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`authorId` varchar(255) NOT NULL,
	`content` text NOT NULL,
	`image_url` text,
	`created_at` timestamp NOT NULL DEFAULT (now()),
	`updated_at` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `Posts_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizAchievement` (
	`id` varchar(255) NOT NULL,
	`name` varchar(255) NOT NULL,
	`description` text NOT NULL,
	`icon` varchar(100),
	`rarity` varchar(50) NOT NULL DEFAULT 'common',
	`requirement` text NOT NULL,
	`rewardPoints` int NOT NULL DEFAULT 0,
	`rewardCoins` int NOT NULL DEFAULT 0,
	CONSTRAINT `QuizAchievement_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizAnswer` (
	`id` varchar(255) NOT NULL,
	`questionId` varchar(255) NOT NULL,
	`text` text NOT NULL,
	`answerIndex` int NOT NULL,
	CONSTRAINT `QuizAnswer_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizFriendship` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`friendId` varchar(255) NOT NULL,
	`status` varchar(50) NOT NULL DEFAULT 'pending',
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `QuizFriendship_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizLobby` (
	`id` varchar(255) NOT NULL,
	`lobbyCode` varchar(8) NOT NULL,
	`hostId` varchar(255) NOT NULL,
	`isPrivate` int NOT NULL DEFAULT 1,
	`maxPlayers` int NOT NULL DEFAULT 2,
	`status` varchar(50) NOT NULL DEFAULT 'waiting',
	`difficulty` varchar(50) NOT NULL,
	`category` varchar(100) NOT NULL,
	`totalQuestions` int NOT NULL DEFAULT 5,
	`timePerQuestion` int NOT NULL DEFAULT 10,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`expiresAt` timestamp NOT NULL,
	CONSTRAINT `QuizLobby_id` PRIMARY KEY(`id`),
	CONSTRAINT `QuizLobby_lobbyCode_unique` UNIQUE(`lobbyCode`)
);
--> statement-breakpoint
CREATE TABLE `QuizLobbyMember` (
	`id` varchar(255) NOT NULL,
	`lobbyId` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`isHost` int NOT NULL DEFAULT 0,
	`isReady` int NOT NULL DEFAULT 0,
	`joinedAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `QuizLobbyMember_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizMatchAnswer` (
	`id` varchar(255) NOT NULL,
	`matchId` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`questionId` varchar(255) NOT NULL,
	`questionIndex` int NOT NULL,
	`answerIndex` int NOT NULL,
	`isCorrect` int NOT NULL,
	`answerTime` int NOT NULL,
	`points` int NOT NULL DEFAULT 0,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `QuizMatchAnswer_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizMatchQuestion` (
	`id` varchar(255) NOT NULL,
	`matchId` varchar(255) NOT NULL,
	`questionId` varchar(255) NOT NULL,
	`questionIndex` int NOT NULL,
	CONSTRAINT `QuizMatchQuestion_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizMatch` (
	`id` varchar(255) NOT NULL,
	`player1Id` varchar(255) NOT NULL,
	`player2Id` varchar(255) NOT NULL,
	`winnerId` varchar(255),
	`gameMode` varchar(50) NOT NULL,
	`difficulty` varchar(50) NOT NULL,
	`category` varchar(100) NOT NULL,
	`status` varchar(50) NOT NULL DEFAULT 'waiting',
	`player1Score` int NOT NULL DEFAULT 0,
	`player2Score` int NOT NULL DEFAULT 0,
	`player1Health` int NOT NULL DEFAULT 100,
	`player2Health` int NOT NULL DEFAULT 100,
	`currentQuestionIndex` int NOT NULL DEFAULT 0,
	`totalQuestions` int NOT NULL DEFAULT 5,
	`timePerQuestion` int NOT NULL DEFAULT 10,
	`startedAt` timestamp,
	`finishedAt` timestamp,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `QuizMatch_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizNotification` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`type` varchar(50) NOT NULL,
	`title` varchar(255) NOT NULL,
	`message` text NOT NULL,
	`data` text,
	`isRead` int NOT NULL DEFAULT 0,
	`priority` varchar(50) NOT NULL DEFAULT 'medium',
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`expiresAt` timestamp,
	CONSTRAINT `QuizNotification_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizQuestion` (
	`id` varchar(255) NOT NULL,
	`text` text NOT NULL,
	`category` varchar(100) NOT NULL,
	`difficulty` varchar(50) NOT NULL,
	`correctAnswer` int NOT NULL,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `QuizQuestion_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizUserAchievement` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`achievementId` varchar(255) NOT NULL,
	`unlockedAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `QuizUserAchievement_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `QuizUserStats` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`points` int NOT NULL DEFAULT 0,
	`wins` int NOT NULL DEFAULT 0,
	`losses` int NOT NULL DEFAULT 0,
	`draws` int NOT NULL DEFAULT 0,
	`totalGames` int NOT NULL DEFAULT 0,
	`currentStreak` int NOT NULL DEFAULT 0,
	`bestStreak` int NOT NULL DEFAULT 0,
	`totalCorrectAnswers` int NOT NULL DEFAULT 0,
	`totalQuestions` int NOT NULL DEFAULT 0,
	`level` int NOT NULL DEFAULT 1,
	`experience` int NOT NULL DEFAULT 0,
	`coins` int NOT NULL DEFAULT 0,
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `QuizUserStats_id` PRIMARY KEY(`id`),
	CONSTRAINT `QuizUserStats_userId_unique` UNIQUE(`userId`)
);
--> statement-breakpoint
CREATE TABLE `Replies` (
	`id` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`commentId` varchar(255) NOT NULL,
	`content` text NOT NULL,
	`created_at` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `Replies_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `RolePermission` (
	`roleId` varchar(255) NOT NULL,
	`permissionId` varchar(255) NOT NULL,
	CONSTRAINT `RolePermission_roleId_permissionId_pk` PRIMARY KEY(`roleId`,`permissionId`)
);
--> statement-breakpoint
CREATE TABLE `Role` (
	`id` varchar(255) NOT NULL,
	`name` varchar(255) NOT NULL,
	`description` text,
	CONSTRAINT `Role_id` PRIMARY KEY(`id`),
	CONSTRAINT `Role_name_unique` UNIQUE(`name`)
);
--> statement-breakpoint
CREATE TABLE `Session` (
	`id` varchar(255) NOT NULL,
	`sessionToken` varchar(255) NOT NULL,
	`userId` varchar(255) NOT NULL,
	`expires` timestamp NOT NULL,
	CONSTRAINT `Session_id` PRIMARY KEY(`id`),
	CONSTRAINT `Session_sessionToken_unique` UNIQUE(`sessionToken`)
);
--> statement-breakpoint
CREATE TABLE `UserRole` (
	`userId` varchar(255) NOT NULL,
	`roleId` varchar(255) NOT NULL,
	CONSTRAINT `UserRole_userId_roleId_pk` PRIMARY KEY(`userId`,`roleId`)
);
--> statement-breakpoint
CREATE TABLE `User` (
	`id` varchar(255) NOT NULL,
	`name` varchar(255),
	`email` varchar(255),
	`emailVerified` timestamp,
	`image` text,
	`password` varchar(255),
	`refreshToken` text,
	`role` varchar(50) NOT NULL DEFAULT 'user',
	CONSTRAINT `User_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
ALTER TABLE `Account` ADD CONSTRAINT `Account_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `ChatMessage` ADD CONSTRAINT `ChatMessage_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `ChatMessage_room` ADD CONSTRAINT `ChatMessage_room_roomId_ChatRoom_id_fk` FOREIGN KEY (`roomId`) REFERENCES `ChatRoom`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `ChatMessage_room` ADD CONSTRAINT `ChatMessage_room_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `ChatRoomMember` ADD CONSTRAINT `ChatRoomMember_roomId_ChatRoom_id_fk` FOREIGN KEY (`roomId`) REFERENCES `ChatRoom`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `ChatRoomMember` ADD CONSTRAINT `ChatRoomMember_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Comments` ADD CONSTRAINT `Comments_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Comments` ADD CONSTRAINT `Comments_postId_Posts_id_fk` FOREIGN KEY (`postId`) REFERENCES `Posts`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Comments` ADD CONSTRAINT `Comments_authorId_User_id_fk` FOREIGN KEY (`authorId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `EmailVerificationToken` ADD CONSTRAINT `EmailVerificationToken_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Likes` ADD CONSTRAINT `Likes_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Likes` ADD CONSTRAINT `Likes_postId_Posts_id_fk` FOREIGN KEY (`postId`) REFERENCES `Posts`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `PasswordResetToken` ADD CONSTRAINT `PasswordResetToken_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Posts` ADD CONSTRAINT `Posts_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Posts` ADD CONSTRAINT `Posts_authorId_User_id_fk` FOREIGN KEY (`authorId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizAnswer` ADD CONSTRAINT `QuizAnswer_questionId_QuizQuestion_id_fk` FOREIGN KEY (`questionId`) REFERENCES `QuizQuestion`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizFriendship` ADD CONSTRAINT `QuizFriendship_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizFriendship` ADD CONSTRAINT `QuizFriendship_friendId_User_id_fk` FOREIGN KEY (`friendId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizLobby` ADD CONSTRAINT `QuizLobby_hostId_User_id_fk` FOREIGN KEY (`hostId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizLobbyMember` ADD CONSTRAINT `QuizLobbyMember_lobbyId_QuizLobby_id_fk` FOREIGN KEY (`lobbyId`) REFERENCES `QuizLobby`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizLobbyMember` ADD CONSTRAINT `QuizLobbyMember_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatchAnswer` ADD CONSTRAINT `QuizMatchAnswer_matchId_QuizMatch_id_fk` FOREIGN KEY (`matchId`) REFERENCES `QuizMatch`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatchAnswer` ADD CONSTRAINT `QuizMatchAnswer_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatchAnswer` ADD CONSTRAINT `QuizMatchAnswer_questionId_QuizQuestion_id_fk` FOREIGN KEY (`questionId`) REFERENCES `QuizQuestion`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatchQuestion` ADD CONSTRAINT `QuizMatchQuestion_matchId_QuizMatch_id_fk` FOREIGN KEY (`matchId`) REFERENCES `QuizMatch`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatchQuestion` ADD CONSTRAINT `QuizMatchQuestion_questionId_QuizQuestion_id_fk` FOREIGN KEY (`questionId`) REFERENCES `QuizQuestion`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatch` ADD CONSTRAINT `QuizMatch_player1Id_User_id_fk` FOREIGN KEY (`player1Id`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatch` ADD CONSTRAINT `QuizMatch_player2Id_User_id_fk` FOREIGN KEY (`player2Id`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizMatch` ADD CONSTRAINT `QuizMatch_winnerId_User_id_fk` FOREIGN KEY (`winnerId`) REFERENCES `User`(`id`) ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizNotification` ADD CONSTRAINT `QuizNotification_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizUserAchievement` ADD CONSTRAINT `QuizUserAchievement_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizUserAchievement` ADD CONSTRAINT `QuizUserAchievement_achievementId_QuizAchievement_id_fk` FOREIGN KEY (`achievementId`) REFERENCES `QuizAchievement`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `QuizUserStats` ADD CONSTRAINT `QuizUserStats_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Replies` ADD CONSTRAINT `Replies_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Replies` ADD CONSTRAINT `Replies_commentId_Comments_id_fk` FOREIGN KEY (`commentId`) REFERENCES `Comments`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `RolePermission` ADD CONSTRAINT `RolePermission_roleId_Role_id_fk` FOREIGN KEY (`roleId`) REFERENCES `Role`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `RolePermission` ADD CONSTRAINT `RolePermission_permissionId_Permission_id_fk` FOREIGN KEY (`permissionId`) REFERENCES `Permission`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `Session` ADD CONSTRAINT `Session_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `UserRole` ADD CONSTRAINT `UserRole_userId_User_id_fk` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE `UserRole` ADD CONSTRAINT `UserRole_roleId_Role_id_fk` FOREIGN KEY (`roleId`) REFERENCES `Role`(`id`) ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX `userId_idx` ON `Account` (`userId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `ChatMessage` (`userId`);--> statement-breakpoint
CREATE INDEX `timestamp_idx` ON `ChatMessage` (`timestamp`);--> statement-breakpoint
CREATE INDEX `roomId_idx` ON `ChatMessage_room` (`roomId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `ChatMessage_room` (`userId`);--> statement-breakpoint
CREATE INDEX `createdAt_idx` ON `ChatMessage_room` (`createdAt`);--> statement-breakpoint
CREATE INDEX `roomId_idx` ON `ChatRoomMember` (`roomId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `ChatRoomMember` (`userId`);--> statement-breakpoint
CREATE INDEX `createdAt_idx` ON `ChatRoom` (`createdAt`);--> statement-breakpoint
CREATE INDEX `postId_idx` ON `Comments` (`postId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `Comments` (`userId`);--> statement-breakpoint
CREATE INDEX `authorId_idx` ON `Comments` (`authorId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `EmailVerificationToken` (`userId`);--> statement-breakpoint
CREATE INDEX `token_idx` ON `EmailVerificationToken` (`token`);--> statement-breakpoint
CREATE INDEX `originalUrl_idx` ON `ImageCache` (`originalUrl`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `PasswordResetToken` (`userId`);--> statement-breakpoint
CREATE INDEX `token_idx` ON `PasswordResetToken` (`token`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `Posts` (`userId`);--> statement-breakpoint
CREATE INDEX `authorId_idx` ON `Posts` (`authorId`);--> statement-breakpoint
CREATE INDEX `created_at_idx` ON `Posts` (`created_at`);--> statement-breakpoint
CREATE INDEX `rarity_idx` ON `QuizAchievement` (`rarity`);--> statement-breakpoint
CREATE INDEX `questionId_idx` ON `QuizAnswer` (`questionId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `QuizFriendship` (`userId`);--> statement-breakpoint
CREATE INDEX `friendId_idx` ON `QuizFriendship` (`friendId`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `QuizFriendship` (`status`);--> statement-breakpoint
CREATE INDEX `lobbyCode_idx` ON `QuizLobby` (`lobbyCode`);--> statement-breakpoint
CREATE INDEX `hostId_idx` ON `QuizLobby` (`hostId`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `QuizLobby` (`status`);--> statement-breakpoint
CREATE INDEX `lobbyId_idx` ON `QuizLobbyMember` (`lobbyId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `QuizLobbyMember` (`userId`);--> statement-breakpoint
CREATE INDEX `matchId_idx` ON `QuizMatchAnswer` (`matchId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `QuizMatchAnswer` (`userId`);--> statement-breakpoint
CREATE INDEX `matchId_idx` ON `QuizMatchQuestion` (`matchId`);--> statement-breakpoint
CREATE INDEX `player1Id_idx` ON `QuizMatch` (`player1Id`);--> statement-breakpoint
CREATE INDEX `player2Id_idx` ON `QuizMatch` (`player2Id`);--> statement-breakpoint
CREATE INDEX `status_idx` ON `QuizMatch` (`status`);--> statement-breakpoint
CREATE INDEX `createdAt_idx` ON `QuizMatch` (`createdAt`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `QuizNotification` (`userId`);--> statement-breakpoint
CREATE INDEX `isRead_idx` ON `QuizNotification` (`isRead`);--> statement-breakpoint
CREATE INDEX `createdAt_idx` ON `QuizNotification` (`createdAt`);--> statement-breakpoint
CREATE INDEX `category_idx` ON `QuizQuestion` (`category`);--> statement-breakpoint
CREATE INDEX `difficulty_idx` ON `QuizQuestion` (`difficulty`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `QuizUserAchievement` (`userId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `QuizUserStats` (`userId`);--> statement-breakpoint
CREATE INDEX `points_idx` ON `QuizUserStats` (`points`);--> statement-breakpoint
CREATE INDEX `commentId_idx` ON `Replies` (`commentId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `Replies` (`userId`);--> statement-breakpoint
CREATE INDEX `userId_idx` ON `Session` (`userId`);--> statement-breakpoint
CREATE INDEX `sessionToken_idx` ON `Session` (`sessionToken`);--> statement-breakpoint
CREATE INDEX `email_idx` ON `User` (`email`);