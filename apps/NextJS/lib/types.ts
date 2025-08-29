export interface User {
  id: string;
  email: string;
  name: string | null;
  image: string | null;
  role: string;
}

export interface DB {
  User: User;
  // Add other tables here as you migrate them from Prisma
}
