"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.prisma = void 0;
var _client = require("@prisma/client");
// lib/prisma.ts

const globalForPrisma = global;
const prisma = exports.prisma = globalForPrisma.prisma || new _client.PrismaClient({
  log: ['query']
});
if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma;