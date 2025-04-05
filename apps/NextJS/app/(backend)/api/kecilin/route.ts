import { NextResponse } from 'next/server';
import logger from '@/lib/logger';

// Temporary type extension for Node.js fetch
declare global {
  interface RequestInit {
    duplex?: 'half' | 'full';
  }
}

export async function POST(request: Request) {
    try {
        const apiUrl = 'https://staging.kecilin.id/api/upload_compress';

        // Clone headers from original request
        const headers = new Headers(request.headers);

        // Add required duplex option with type safety
        const init: RequestInit = {
            method: 'POST',
            headers: headers,
            body: request.body,
            duplex: 'half' 
        };

        const response = await fetch(apiUrl, init);

        if (!response.ok) {
            throw new Error(`API request failed: ${response.status} ${response.statusText}`);
        }

        const responseData = await response.json();

        return NextResponse.json(responseData);
    } catch (error) {
        logger.error("Proxy error:", error);
        return NextResponse.json(
            { status: 500, message: "Internal Server Error" },
            { status: 500 }
        );
    }
}

export const config = {
    api: {
        bodyParser: false,
    },
};