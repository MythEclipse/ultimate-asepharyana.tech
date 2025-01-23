"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
var _exportNames = {
  prisma: true
};
exports.prisma = void 0;
var _client = require("@prisma/client");
Object.keys(_client).forEach(function (key) {
  if (key === "default" || key === "__esModule") return;
  if (Object.prototype.hasOwnProperty.call(_exportNames, key)) return;
  if (key in exports && exports[key] === _client[key]) return;
  Object.defineProperty(exports, key, {
    enumerable: true,
    get: function () {
      return _client[key];
    }
  });
});
const prisma = exports.prisma = global.prisma || new _client.PrismaClient();
if (process.env.NODE_ENV !== "production") global.prisma = prisma;