import { auth } from "./auth"
import { NextResponse } from "next/server";

export default auth((req) => {
  if (
    !req.auth &&
    (req.nextUrl.pathname.startsWith("/sosmed") || req.nextUrl.pathname.startsWith("/chat"))
  ) {
    const newUrl = new URL("/login", req.nextUrl.origin)
    return Response.redirect(newUrl)
  }
  return NextResponse.next();
})

export const config = {
  matcher: ["/sosmed/:path*", "/chat/:path*"],
}
