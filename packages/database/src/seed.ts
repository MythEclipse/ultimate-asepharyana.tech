// import { prisma } from "./client";



// // const DEFAULT_USERS = [
// //   {
// //     name: "Asep Haryana",
// //     email: "asep.haryana@gmail.com",
// //   },
// //   {
// //     name: "Demo User",
// //     email: "demo@example.com",
// //   },
// // ] as Array<Partial<User>>;

// (async () => {
//   try {
//     await Promise.all(
//       DEFAULT_USERS.map((user) =>
//         prisma.user.upsert({
//           where: {
//             email: user.email!,
//           },
//           update: {
//             ...user,
//           },
//           create: {
//             ...user,
//           },
//         }),
//       ),
//     );
//     console.log("Seed completed successfully");
//   } catch (error) {
//     console.error(error);
//     process.exit(1);
//   } finally {
//     await prisma.$disconnect();
//   }
// })();
