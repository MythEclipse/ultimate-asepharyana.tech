"use client";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { FallbackProps } from "react-error-boundary";

export function ErrorFallback({ error, resetErrorBoundary }: FallbackProps) {
  return (
    <div className="flex items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900" role="alert">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle className="text-red-600 dark:text-red-400">Something went wrong.</CardTitle>
          <CardDescription>An unexpected error occurred. Please try again.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="font-semibold">Error Details:</p>
          <pre className="p-4 bg-gray-50 dark:bg-gray-800 rounded-md overflow-auto text-sm">
            <code>{error.message}</code>
          </pre>
        </CardContent>
        <CardFooter className="flex justify-end gap-2">
          <Button onClick={resetErrorBoundary} variant="outline">
            Try again
          </Button>
          <Button onClick={() => window.location.reload()}>
            Refresh Page
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}