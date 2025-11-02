import { PrismaClient } from '@prisma/client';

async function main() {
  const prisma = new PrismaClient();
  try {
    console.log('Connecting and listing tables...');
    const rows = await prisma.$queryRaw<any[]>`SHOW TABLES`;
    console.log('Tables:', rows);
  } catch (err) {
    console.error('Error querying tables:', err);
  } finally {
    await prisma.$disconnect();
  }
}

main();
