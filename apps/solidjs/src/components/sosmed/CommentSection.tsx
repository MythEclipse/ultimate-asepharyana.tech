import { createSignal, For } from 'solid-js';
import { useAuth } from '~/lib/auth-context';
import { httpClient } from '~/lib/http-client';
import { Comment } from '~/types/sosmed';
import { formatDistanceToNow } from 'date-fns';

interface CommentSectionProps {
    postId: string;
    comments: Comment[];
    onCommentAdded: (comment: Comment) => void;
}

export default function CommentSection(props: CommentSectionProps) {
    const { user } = useAuth();
    const [content, setContent] = createSignal('');
    const [isLoading, setIsLoading] = createSignal(false);

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        if (!content().trim()) return;

        setIsLoading(true);

        try {
            const response = await httpClient.request<{ success: boolean; comment: Comment }>(
                `/api/sosmed/posts/${props.postId}/comments`,
                'POST',
                { content: content() }
            );

            if (response.success) {
                props.onCommentAdded(response.comment);
                setContent('');
            }
        } catch (err) {
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div class="mt-4 pt-4 border-t border-border/50">
            <div class="space-y-4 mb-4">
                <For each={props.comments}>
                    {(comment) => (
                        <div class="flex gap-3">
                            <img
                                src={comment.user.image || `https://ui-avatars.com/api/?name=${comment.user.name}`}
                                alt={comment.user.name || 'User'}
                                class="w-8 h-8 rounded-full object-cover"
                            />
                            <div class="flex-grow bg-muted/30 rounded-lg p-3">
                                <div class="flex justify-between items-start mb-1">
                                    <span class="font-semibold text-sm">{comment.user.name}</span>
                                    <span class="text-xs text-muted-foreground">
                                        {formatDistanceToNow(new Date(comment.created_at), { addSuffix: true })}
                                    </span>
                                </div>
                                <p class="text-sm text-foreground/90">{comment.content}</p>
                            </div>
                        </div>
                    )}
                </For>
            </div>

            <form onSubmit={handleSubmit} class="flex gap-3">
                <img
                    src={user()?.image || `https://ui-avatars.com/api/?name=${user()?.name || 'User'}`}
                    alt={user()?.name || 'User'}
                    class="w-8 h-8 rounded-full object-cover"
                />
                <div class="flex-grow flex gap-2">
                    <input
                        type="text"
                        value={content()}
                        onInput={(e) => setContent(e.currentTarget.value)}
                        placeholder="Write a comment..."
                        class="flex-grow bg-background/50 border border-border rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 transition-all"
                    />
                    <button
                        type="submit"
                        disabled={isLoading() || !content().trim()}
                        class="px-4 py-2 bg-secondary text-secondary-foreground rounded-lg text-sm font-medium hover:bg-secondary/80 transition-colors disabled:opacity-50"
                    >
                        Post
                    </button>
                </div>
            </form>
        </div>
    );
}
