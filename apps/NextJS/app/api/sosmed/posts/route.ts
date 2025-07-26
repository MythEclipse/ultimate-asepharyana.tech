import { NextResponse } from 'next/server';
import { prisma } from '@asepharyana/database';
import { getAuthenticatedUser } from '@/lib/authUtils'; // Import getAuthenticatedUser

// Initialize Prisma Client

export async function POST(request: Request) {
  try {
    // Get authenticated user
    const user = await getAuthenticatedUser();
    const userId = user?.id;

    // Parse the JSON request body
    const { content, imageUrl } = await request.json();

    // Validate request data
    if (!content || typeof content !== 'string') {
      return NextResponse.json(
        { message: 'Content is required and must be a string' },
        { status: 400 }
      );
    }

    // Check if userId is available
    if (!userId) {
      return NextResponse.json(
        { message: 'User not authenticated' },
        { status: 401 }
      );
    }

    // Create post in the database
    const newPost = await prisma.posts.create({
      data: {
        content,
        authorId: userId,
        image_url: imageUrl || '',
        userId,
      },
    });

    return NextResponse.json(
      { message: 'Post created successfully!', post: newPost },
      { status: 201 }
    );
// eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (error) {
    // console.error('Error creating post:', error);
    return NextResponse.json(
      { message: 'Failed to create post' },
      { status: 500 }
    );
  } 
}

/**
 * Fetches all posts from the database, including associated user, comments, and likes.
 * The posts are ordered by creation date in descending order.
 * Each post's comments are enriched with user information.
 *
 * @returns {Promise<NextResponse>} A JSON response containing the sanitized posts or an error message.
 *
 * @throws {Error} If there is an error fetching the posts from the database.
 */
export async function GET() {
  try {
    // Fetch all posts from the database
    const posts = await prisma.posts.findMany({
      include: {
        user: {
          select: {
            id: true,
            name: true,
            image: true,
          },
        },
        comments: true,
        likes: true,
      },
      orderBy: {
        created_at: 'desc',
      },
    });

    // Sanitize the response
    const sanitizedPosts = await Promise.all(
      posts.map(
        async (post: {
          id: string;
          content: string;
          user: { id: string; name: string | null; image: string | null };
          comments: { id: string; content: string; userId: string }[];
          likes: { userId: string; postId: string }[];
        }) => {
          const commentsWithUser = await Promise.all(
            post.comments.map(async (comment) => {
              const user = await prisma.user.findUnique({
                where: { id: comment.userId },
                select: { id: true, name: true, image: true },
              });
              return {
                ...comment,
                user,
              };
            })
          );

          return {
            ...post,
            user: {
              id: post.user.id,
              name: post.user.name,
              image: post.user.image,
            },
            comments: commentsWithUser,
            likes: post.likes.map((like) => ({
              userId: like.userId,
              postId: like.postId,
            })),
          };
        }
      )
    );

// eslint-disable-next-line @typescript-eslint/no-unused-vars
    return NextResponse.json({ posts: sanitizedPosts }, { status: 200 });
  } catch (error) {
    // console.error('Error fetching posts:', error);
    return NextResponse.json(
      { message: 'Failed to fetch posts' },
      { status: 500 }
    );
  } 
}

export async function PUT(request: Request) {
  try {
    // Get authenticated user
    const user = await getAuthenticatedUser();
    const userId = user?.id;

    if (!user || !userId) {
      return NextResponse.json(
        { message: 'User not authenticated' },
        { status: 401 }
      );
    }
    // Parse the JSON request body
    const { id, content } = await request.json();

    // Validate request data
    if (!id || !content || typeof content !== 'string') {
      return NextResponse.json(
        { message: 'Post ID and content are required and must be valid' },
        { status: 400 }
      );
    }

    // Check if userId is available
    if (!userId) {
      return NextResponse.json(
        { message: 'User not authenticated' },
        { status: 401 }
      );
    }

    // Fetch the post to check ownership
    const post = await prisma.posts.findUnique({ where: { id } });

    if (!post || post.userId !== userId) {
      return NextResponse.json(
        { message: 'User not authorized to edit this post' },
        { status: 403 }
      );
    }

    // Update post in the database
    const updatedPost = await prisma.posts.update({
      where: { id },
      data: {
        content,
      },
    });

    return NextResponse.json(
      { message: 'Post updated successfully!', post: updatedPost },
// eslint-disable-next-line @typescript-eslint/no-unused-vars
      { status: 200 }
    );
  } catch (error) {
    // console.error('Error updating post:', error);
    return NextResponse.json(
      { message: 'Failed to update post' },
      { status: 500 }
    );
  } 
}

export async function DELETE(request: Request) {
  try {
    // Get authenticated user
    const user = await getAuthenticatedUser();
    const userId = user?.id;

    // Validate authentication
    if (!user || !userId) {
      return NextResponse.json(
        { message: 'User not authenticated' },
        { status: 401 }
      );
    }

    // Parse the JSON request body
    const { id } = await request.json();

    // Validate request data
    if (!id) {
      return NextResponse.json(
        { message: 'Post ID is required' },
        { status: 400 }
      );
    }

    // Fetch the post to check ownership
    const post = await prisma.posts.findUnique({ where: { id } });

    if (!post) {
      return NextResponse.json({ message: 'Post not found' }, { status: 404 });
    }

    if (post.userId !== userId) {
      return NextResponse.json(
        { message: 'User not authorized to delete this post' },
        { status: 403 }
      );
    }

    // Delete post from the database
    await prisma.posts.delete({ where: { id } });

    return NextResponse.json(
// eslint-disable-next-line @typescript-eslint/no-unused-vars
      { message: 'Post deleted successfully!' },
      { status: 200 }
    );
  } catch (error) {
    // console.error('Error deleting post:', error);
    return NextResponse.json(
      { message: 'Failed to delete post' },
      { status: 500 }
    );
  } 
}
