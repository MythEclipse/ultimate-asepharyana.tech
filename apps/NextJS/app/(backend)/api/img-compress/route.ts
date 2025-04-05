import { NextResponse, NextRequest } from 'next/server';
import logger from '@/lib/logger';
import { BaseUrl, PRODUCTION } from '@/lib/url';

declare global {
    interface RequestInit {
        duplex?: 'half' | 'full';
    }
}

const constructUrl = (base: string, path: string) =>
    new URL(path, base.endsWith('/') ? base.slice(0, -1) : base).toString();

const ALLOWED_HEADERS = ['content-type', 'content-length'];

async function compressImage(request: NextRequest) {
    const abortController = new AbortController();
    const timeoutId = setTimeout(() => abortController.abort(), 10000);

    try {
        const apiUrl = 'https://staging.kecilin.id/api/upload_compress';

        const headers = new Headers();
        ALLOWED_HEADERS.forEach(header => {
            const value = request.headers.get(header);
            if (value) headers.set(header, value);
        });

        const init: RequestInit = {
            method: 'POST',
            headers,
            body: request.body,
            duplex: 'half',
            signal: abortController.signal,
        };

        const response = await fetch(apiUrl, init);
        clearTimeout(timeoutId);

        if (!response.ok) {
            const errorBody = await response.text();
            logger.error(`API Error ${response.status}: ${errorBody}`);
            throw new Error(`API request failed: ${response.status}`);
        }

        const contentType = response.headers.get('content-type');
        if (!contentType?.includes('application/json')) {
            throw new Error('Invalid response format');
        }

        const responseData = await response.json();

        if (!responseData.data?.filename) {
            throw new Error('Invalid response structure');
        }

        return {
            status: responseData.status,
            message: responseData.message,
            data: {
                size_ori: responseData.data.size_ori,
                compress_size: responseData.data.compress_size,
                filename: responseData.data.filename,
                link: constructUrl(
                    PRODUCTION,
                    `/api/img-compress?url=${encodeURIComponent(responseData.data.filename)}`
                ),
            },
        };
    } catch (error) {
        clearTimeout(timeoutId);
        logger.error('Compress Image Error:', error);
        throw error;
    }
}

export async function POST(request: NextRequest) {
    try {
        const result = await compressImage(request);
        return NextResponse.json(result);
    } catch (error) {
        const status =
            error instanceof Error && error.message.includes('API request failed')
                ? Number(error.message.split(': ')[1]) || 500
                : 500;

        return NextResponse.json(
            { status, message: 'Proxy error occurred' },
            { status }
        );
    }
}

export async function GET(request: NextRequest) {
    const abortController = new AbortController();
    const timeoutId = setTimeout(() => abortController.abort(), 10000);

    try {
        const { searchParams } = new URL(request.url);
        const fileUrl = searchParams.get('url');

        if (!fileUrl) {
            return NextResponse.json(
                { status: 400, message: 'Missing url parameter' },
                { status: 400 }
            );
        }

        try {
            new URL(fileUrl);
        } catch {
            return NextResponse.json(
                { status: 400, message: 'Invalid URL format' },
                { status: 400 }
            );
        }

        const apiUrl = constructUrl(BaseUrl, '/api/img-compress');

        const init: RequestInit = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ url: fileUrl }),
            signal: abortController.signal,
        };

        const response = await fetch(apiUrl, init);
        clearTimeout(timeoutId);

        if (!response.ok) {
            const errorBody = await response.text();
            logger.error(`Backend Error ${response.status}: ${errorBody}`);
            throw new Error(`Backend request failed: ${response.status}`);
        }

        const responseData = await response.json();
        return NextResponse.json(responseData);
    } catch (error) {
        clearTimeout(timeoutId);
        logger.error('Proxy Error:', error);

        const status =
            error instanceof Error && error.message.includes('Backend request failed')
                ? Number(error.message.split(': ')[1]) || 500
                : 500;

        return NextResponse.json(
            { status, message: 'Error processing image URL' },
            { status }
        );
    }
}

export const config = {
    api: {
        bodyParser: false,
    },
};
