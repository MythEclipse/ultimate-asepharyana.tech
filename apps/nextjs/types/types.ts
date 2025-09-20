/**
 * @fileoverview Core application types, API response shapes, and component prop interfaces.
 */

// Core application types
/**
 * Represents a client-side user with basic profile information and role.
 * @interface ClientUser
 * @property {string} id - Unique identifier for the user.
 * @property {string | null} name - The user's display name.
 * @property {string | null} email - The user's email address.
 * @property {string | null} image - URL to the user's profile image.
 * @property {Date | null} emailVerified - Timestamp when the email was verified.
 * @property {string} role - The user's role (e.g., 'user', 'admin').
 */
export interface ClientUser {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
  emailVerified: Date | null;
  role: string;
}

// API response shapes
/**
 * Generic interface for API responses.
 * @template T - The type of data contained in the response.
 * @interface ApiResponse
 * @property {T} data - The main data payload of the response.
 * @property {string} [error] - Optional error message if the request failed.
 * @property {Date} timestamp - The timestamp when the response was generated.
 */
export interface ApiResponse<T> {
  data: T;
  error?: string;
  timestamp: Date;
}

// Component props
/**
 * Defines the structure for navigation links used across the application.
 * @interface NavLink
 * @property {string} href - The destination URL for the link.
 * @property {string} label - The display text for the link.
 */
export interface NavLink {
  href: string;
  label: string;
}

/**
 * Props for the MobileNav component.
 * @interface MobileNavProps
 * @property {boolean} isOpen - Indicates whether the mobile navigation is open.
 * @property {(isOpen: boolean) => void} setIsOpen - Function to set the open state of the mobile navigation.
 */
export interface MobileNavProps {
  isOpen: boolean;
  setIsOpen: (isOpen: boolean) => void;
}

// Social media module types
/**
 * Represents a social media post.
 * @interface PostType
 * @property {string} id - Unique identifier for the post.
 * @property {string} content - The main text content of the post.
 * @property {string} [imageUrl] - Optional URL to an image associated with the post.
 * @property {Date} createdAt - Timestamp when the post was created.
 * @property {ClientUser} user - The user who created the post.
 * @property {LikeType[]} likes - An array of likes associated with the post.
 * @property {CommentType[]} comments - An array of comments associated with the post.
 */
export interface PostType {
  id: string;
  content: string;
  imageUrl?: string;
  createdAt: Date;
  user: ClientUser;
  likes: LikeType[];
  comments: CommentType[];
}

/**
 * Represents a comment on a social media post.
 * @interface CommentType
 * @property {string} id - Unique identifier for the comment.
 * @property {string} content - The text content of the comment.
 * @property {Date} createdAt - Timestamp when the comment was created.
 * @property {ClientUser} user - The user who created the comment.
 */
export interface CommentType {
  id: string;
  content: string;
  createdAt: Date;
  user: ClientUser;
}

/**
 * Represents a like on a social media post.
 * @interface LikeType
 * @property {string} id - Unique identifier for the like.
 * @property {string} userId - The ID of the user who liked the post.
 * @property {Date} createdAt - Timestamp when the like was created.
 */
export interface LikeType {
  id: string;
  userId: string;
  createdAt: Date;
}
