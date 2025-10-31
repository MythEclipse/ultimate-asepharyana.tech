// Auth types for Rust backend integration
export interface User {
  id: string;
  email: string;
  name: string;
  image?: string | null;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterData {
  email: string;
  name: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}
