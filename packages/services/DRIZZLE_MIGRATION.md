# Migration Guide: Kysely to Drizzle ORM

## Changes Made

### 1. Dependencies
**Before:**
```json
"kysely": "^0.28.8",
"mysql2": "^3.15.3"
```

**After:**
```json
"drizzle-orm": "^0.36.4",
"mysql2": "^3.15.3",
"drizzle-kit": "^0.29.1" (devDependencies)
```

### 2. Schema Definition
Created `src/lib/schema.ts` with Drizzle schema definitions:
- Type-safe table definitions using `mysqlTable`
- Relations defined using `relations()` helper
- Indexes and foreign keys properly configured
- All tables from original types are included

### 3. Database Connection
**Before (Kysely):**
```typescript
import { Kysely, MysqlDialect } from 'kysely';
const db = new Kysely<DB>({ dialect });
```

**After (Drizzle):**
```typescript
import { drizzle } from 'drizzle-orm/mysql2';
import { createPool } from 'mysql2/promise';
const pool = createPool(databaseUrl);
const db = drizzle(pool, { schema });
```

### 4. Types
Types are now inferred from schema using Drizzle helpers:
```typescript
import type { InferSelectModel, InferInsertModel } from 'drizzle-orm';
export type User = InferSelectModel<typeof schema.users>;
export type NewUser = InferInsertModel<typeof schema.users>;
```

## Usage Examples

### Initialize Database
```typescript
import { initializeDb, getDb } from '@asepharyana/services';

// Initialize connection
initializeDb(process.env.DATABASE_URL!);

// Get db instance
const db = getDb();
```

### Query Examples

**Select:**
```typescript
import { getDb } from '@asepharyana/services';
import { users, posts } from '@asepharyana/services';
import { eq } from 'drizzle-orm';

const db = getDb();

// Select all users
const allUsers = await db.select().from(users);

// Select with condition
const user = await db.select().from(users).where(eq(users.id, userId));

// Select with relations
const userWithPosts = await db.query.users.findFirst({
  where: eq(users.id, userId),
  with: {
    posts: true,
    comments: true
  }
});
```

**Insert:**
```typescript
import { users } from '@asepharyana/services';
import type { NewUser } from '@asepharyana/services';

const newUser: NewUser = {
  id: 'user-123',
  name: 'John Doe',
  email: 'john@example.com',
  role: 'user',
  emailVerified: null,
  image: null,
  password: 'hashed-password',
  refreshToken: null
};

await db.insert(users).values(newUser);
```

**Update:**
```typescript
import { users } from '@asepharyana/services';
import { eq } from 'drizzle-orm';

await db.update(users)
  .set({ name: 'Jane Doe' })
  .where(eq(users.id, userId));
```

**Delete:**
```typescript
import { users } from '@asepharyana/services';
import { eq } from 'drizzle-orm';

await db.delete(users).where(eq(users.id, userId));
```

## Drizzle Commands

### Generate Migrations
```bash
cd libs/services
bun run drizzle-kit generate
```

### Push Schema to Database
```bash
bun run drizzle-kit push
```

### Run Migrations
```bash
bun run drizzle-kit migrate
```

### Drizzle Studio (Database GUI)
```bash
bun run drizzle-kit studio
```

## Migration Checklist for Applications

If your applications (elysia, nextjs, rust) are using this library:

1. **Update imports:**
   - Replace Kysely query builders with Drizzle query builders
   - Import table schemas from `@asepharyana/services`
   - Import operators from `drizzle-orm` (eq, and, or, like, etc.)

2. **Update queries:**
   - Kysely's `.selectFrom()` becomes `.select().from()`
   - Kysely's `.where()` uses different operator functions
   - Relations are accessed via `.query` API or joins

3. **Benefits:**
   - Better TypeScript inference
   - More intuitive API
   - Built-in relations support
   - Better migration tooling
   - Active development and community

## Notes

- All existing types are preserved for backward compatibility
- The `DB` interface type is maintained
- Schema includes proper indexes for performance
- Relations enable easy joined queries
- `closeDb()` function added for cleanup
