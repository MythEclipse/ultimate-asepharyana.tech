import Link from 'next/link';

export default function NotFound() {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <div className="text-center">
        <h1 className="mb-4 text-7xl font-bold">404</h1>
        <p className="mb-4 text-xl">Page Not Found</p>
        <Link href="/" className="text-blue-500 hover:underline">
          Go back home
        </Link>
      </div>
    </div>
  );
}
