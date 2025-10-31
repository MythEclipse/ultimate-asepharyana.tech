export interface User {
  id: string;
  email: string;
  username: string;
  password_hash: string;
  full_name: string | null;
  avatar_url: string | null;
  email_verified: boolean;
  is_active: boolean;
  role: string;
  last_login_at: Date | null;
  created_at: Date;
  updated_at: Date;
}

export interface UserResponse {
  id: string;
  email: string;
  username: string;
  full_name: string | null;
  avatar_url: string | null;
  email_verified: boolean;
  is_active: boolean;
  role: string;
  last_login_at: Date | null;
  created_at: Date;
  updated_at: Date;
}

export interface LoginResponse {
  user: UserResponse;
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
}

export interface RegisterResponse {
  success: boolean;
  message: string;
  user: UserResponse;
  verification_token: string | null;
}

export function toUserResponse(user: User): UserResponse {
  return {
    id: user.id,
    email: user.email,
    username: user.username,
    full_name: user.full_name,
    avatar_url: user.avatar_url,
    email_verified: user.email_verified,
    is_active: user.is_active,
    role: user.role,
    last_login_at: user.last_login_at,
    created_at: user.created_at,
    updated_at: user.updated_at,
  };
}
