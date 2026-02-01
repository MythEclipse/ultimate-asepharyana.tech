import { createSignal, Show } from 'solid-js';
import { useAuth } from '~/lib/auth-context';
import { httpClient } from '~/lib/http-client';
import { Post, Comment, Like } from '~/types/sosmed';
import { formatDistanceToNow } from 'date-fns';
import CommentSection from './CommentSection';
import { Motion } from 'solid-motionone';

interface PostCardProps {
    post: Post;
    onPostUpdated: (post: Post) => void;
    onDelete: (postId: string) => void;
}

export default function PostCard(props: PostCardProps) {
    const { user } = useAuth();
    const [showComments, setShowComments] = createSignal(false);
    const [isLiking, setIsLiking] = createSignal(false);

    const isLiked = () => props.post.likes.some(like => like.userId === user()?.id);
    const isOwner = () => props.post.userId === user()?.id;

    const handleLike = async () => {
        if (isLiking()) return;
        setIsLiking(true);

        try {
            const method = isLiked() ? 'DELETE' : 'POST';
            const response = await httpClient.request<{ success: boolean; like?: Like }>(
                `/api/sosmed/posts/${props.post.id}/like`,
                method
            );

            if (response.success) {
                const updatedLikes = isLiked()
                    ? props.post.likes.filter(l => l.userId !== user()?.id)
                    : [...props.post.likes, response.like!];

                props.onPostUpdated({ ...props.post, likes: updatedLikes });
            }
        } catch (err) {
            console.error(err);
        } finally {
            setIsLiking(false);
        }
    };

    const handleCommentAdded = (comment: Comment) => {
        props.onPostUpdated({
            ...props.post,
            comments: [comment, ...props.post.comments]
        });
    };

    const handleDelete = async () => {
        if (!confirm('Are you sure you want to delete this post?')) return;

        try {
            await httpClient.request(`/api/sosmed/posts/${props.post.id}`, 'DELETE');
            props.onDelete(props.post.id);
        } catch (err) {
            console.error(err);
        }
    };

    return (
        <Motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            class="glass-card p-6 rounded-2xl mb-6 last:mb-0"
        >
            {/* Header */}
            <div class="flex justify-between items-start mb-4">
                <div class="flex gap-3 items-center">
                    <img
                        src={props.post.user?.image || `https://ui-avatars.com/api/?name=${props.post.user?.name || 'User'}`}
                        alt={props.post.user?.name || 'User'}
                        class="w-10 h-10 rounded-full object-cover border border-border"
                    />
                    <div>
                        <h3 class="font-semibold">{props.post.user?.name || 'Unknown User'}</h3>
                        <p class="text-xs text-muted-foreground">
                            {formatDistanceToNow(new Date(props.post.created_at), { addSuffix: true })}
                        </p>
                    </div>
                </div>

                <Show when={isOwner()}>
                    <button
                        onClick={handleDelete}
                        class="text-muted-foreground hover:text-destructive transition-colors p-2"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                        </svg>
                    </button>
                </Show>
            </div>

            {/* Content */}
            <p class="text-foreground/90 mb-4 whitespace-pre-wrap">{props.post.content}</p>

            <Show when={props.post.image_url}>
                <div class="mb-4 rounded-xl overflow-hidden border border-border/50">
                    <img
                        src={props.post.image_url!}
                        alt="Post content"
                        class="w-full h-auto max-h-[500px] object-cover"
                    />
                </div>
            </Show>

            {/* Actions */}
            <div class="flex items-center gap-6 pt-4 border-t border-border/50">
                <button
                    onClick={handleLike}
                    disabled={isLiking()}
                    class={`flex items-center gap-2 text-sm font-medium transition-colors ${isLiked() ? 'text-red-500' : 'text-muted-foreground hover:text-red-500'
                        }`}
                >
                    <svg
                        class={`w-5 h-5 ${isLiked() ? 'fill-current' : 'fill-none'}`}
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                    {props.post.likes.length} Likes
                </button>

                <button
                    onClick={() => setShowComments(!showComments())}
                    class="flex items-center gap-2 text-sm font-medium text-muted-foreground hover:text-primary transition-colors"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                    {props.post.comments.length} Comments
                </button>
            </div>

            <Show when={showComments()}>
                <CommentSection
                    postId={props.post.id}
                    comments={props.post.comments}
                    onCommentAdded={handleCommentAdded}
                />
            </Show>
        </Motion.div>
    );
}
