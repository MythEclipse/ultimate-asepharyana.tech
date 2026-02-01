import { createSignal } from 'solid-js';
import { useAuth } from '~/lib/auth-context';
import { httpClient } from '~/lib/http-client';
import { Post } from '~/types/sosmed';

interface CreatePostProps {
    onPostCreated: (post: Post) => void;
}

export default function CreatePost(props: CreatePostProps) {
    const { user } = useAuth();
    const [content, setContent] = createSignal('');
    const [imageUrl, setImageUrl] = createSignal('');
    const [isLoading, setIsLoading] = createSignal(false);
    const [error, setError] = createSignal('');

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        if (!content() && !imageUrl()) return;

        setIsLoading(true);
        setError('');

        try {
            const response = await httpClient.fetchJson<{ success: boolean; post: Post }>(
                '/api/sosmed/posts',
                {
                    method: 'POST',
                    body: JSON.stringify({
                        content: content(),
                        imageUrl: imageUrl() || undefined,
                    }),
                }
            );

            if (response.success) {
                props.onPostCreated(response.post);
                setContent('');
                setImageUrl('');
            }
        } catch (err) {
            console.error(err);
            setError('Failed to create post');
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div class="glass-card p-6 rounded-2xl mb-8">
            <div class="flex gap-4">
                <div class="flex-shrink-0">
                    <img
                        src={user()?.image || `https://ui-avatars.com/api/?name=${user()?.name || 'User'}`}
                        alt={user()?.name || 'User'}
                        class="w-10 h-10 rounded-full object-cover border-2 border-primary/20"
                    />
                </div>
                <div class="flex-grow">
                    <form onSubmit={handleSubmit}>
                        <textarea
                            value={content()}
                            onInput={(e) => setContent(e.currentTarget.value)}
                            placeholder="What's on your mind?"
                            class="w-full bg-background/50 border border-border rounded-xl p-4 min-h-[100px] focus:outline-none focus:ring-2 focus:ring-primary/50 transition-all resize-none mb-4"
                        />

                        <div class="flex items-center gap-4 mb-4">
                            <input
                                type="text"
                                value={imageUrl()}
                                onInput={(e) => setImageUrl(e.currentTarget.value)}
                                placeholder="Image URL (optional)"
                                class="flex-grow bg-background/50 border border-border rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 transition-all"
                            />
                        </div>

                        <div class="flex justify-between items-center">
                            <span class="text-sm text-destructive">{error()}</span>
                            <button
                                type="submit"
                                disabled={isLoading() || (!content() && !imageUrl())}
                                class="px-6 py-2 bg-primary text-primary-foreground rounded-lg font-medium hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                {isLoading() ? 'Posting...' : 'Post'}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    );
}
