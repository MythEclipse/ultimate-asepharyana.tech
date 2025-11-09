import { initializeDb, getDb, closeDb, users, NewUser } from '@asepharyana/services';
import { eq } from 'drizzle-orm';
import bcrypt from 'bcryptjs';
import { config } from '../src/config';

async function main() {
  console.log('🌱 Starting database seeding...');

  initializeDb(config.databaseUrl);
  const db = getDb();

  // Create test user
  const hashedPassword = await bcrypt.hash('Password123!', 10);

  // Check if user exists
  const existingUser = await db.query.users.findFirst({
    where: eq(users.email, 'test@example.com'),
  });

  let user;
  if (!existingUser) {
    const userId = `user_${Date.now()}_test`;
    const newUser: NewUser = {
      id: userId,
      email: 'test@example.com',
      name: 'Test User',
      password: hashedPassword,
      emailVerified: new Date(),
      role: 'user',
    };
    await db.insert(users).values(newUser);
    user = await db.query.users.findFirst({
      where: eq(users.id, userId),
    });
  } else {
    user = existingUser;
  }

  console.log('✅ Created test user:', user?.email);

  // Create another verified user
  const existingAdmin = await db.query.users.findFirst({
    where: eq(users.email, 'admin@example.com'),
  });

  let user2;
  if (!existingAdmin) {
    const userId2 = `user_${Date.now()}_admin`;
    const newAdmin: NewUser = {
      id: userId2,
      email: 'admin@example.com',
      name: 'Admin User',
      password: hashedPassword,
      emailVerified: new Date(),
      role: 'admin',
    };
    await db.insert(users).values(newAdmin);
    user2 = await db.query.users.findFirst({
      where: eq(users.id, userId2),
    });
  } else {
    user2 = existingAdmin;
  }

  console.log('✅ Created admin user:', user2?.email);

  console.log('🌱 Database seeding completed!');
}

main()
  .catch((e) => {
    console.error('❌ Error during seeding:', e);
    process.exit(1);
  })
  .finally(async () => {
    await closeDb();
  });
