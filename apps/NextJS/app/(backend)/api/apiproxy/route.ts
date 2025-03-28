import { NextResponse } from 'next/server';

/**
 * Handles GET requests to proxy API calls to a target URL.
 * 
 * This function extracts the `url` query parameter from the incoming request,
 * fetches data from the specified target URL, and returns the response as JSON.
 * If the `url` parameter is missing or the fetch operation fails, it returns
 * an error response with a status code of 500.
 * 
 * @param request - The incoming HTTP request object.
 * @returns A JSON response containing the fetched data or an error message.
 * 
 * @example
 * // Example usage:
 * // Assuming the API is hosted at `https://example.com/api/apiproxy`
 * // and you want to fetch data from `https://api.example.com/data`:
 * 
 * const response = await fetch('https://example.com/api/apiproxy?url=https://api.example.com/data');
 * const data = await response.json();
 * 
 * if (response.ok) {
 *     console.log('Fetched data:', data);
 * } else {
 *     console.error('Error:', data.error, 'Details:', data.details);
 * }
 * 
 * @throws Will throw an error if the `url` query parameter is missing or if the fetch operation fails.
 */
export async function GET(request: Request) {
    try {
        const { searchParams } = new URL(request.url);
        const targetUrl = searchParams.get('url'); // Extract the full URL from the query parameter

        if (!targetUrl) {
            throw new Error('Missing "url" parameter');
        }

        const response = await fetch(targetUrl); // Use the provided URL directly
        if (!response.ok) {
            throw new Error('Failed to fetch data from the API');
        }

        const data = await response.json();
        return NextResponse.json(data);
    } catch (error) {
        return NextResponse.json(
            { error: 'Failed to fetch data', details: (error as Error).message },
            { status: 500 }
        );
    }
}