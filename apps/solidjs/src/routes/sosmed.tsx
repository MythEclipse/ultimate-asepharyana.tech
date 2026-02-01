import { Title } from '@solidjs/meta';
import { createSignal, createResource, For, Show } from 'solid-js';
import { useAuth } from '~/lib/auth-context';
import { httpClient } from '~/lib/http-client';
import { Post } from '~/types/sosmed';
import CreatePost from '~/components/sosmed/CreatePost';
import PostCard from '~/components/sosmed/PostCard';

const fetchPosts = async () => {
    const response = await httpClient.fetchJson<{ success: boolean; posts: Post[] }>('/api/sosmed/posts');
    return response.success ? response.posts : [];
};

export default function SosmedPage() {
    const { user } = useAuth();
    const [posts, { mutate }] = createResource(fetchPosts);

    const handlePostCreated = (newPost: Post) => {
        mutate((prev) => [newPost, ...(prev || [])]);
    };

    const handlePostUpdated = (updatedPost: Post) => {
        mutate((prev) => prev?.map((p) => (p.id === updatedPost.id ? updatedPost : p)));
    };

    const handlePostDeleted = (postId: string) => {
        mutate((prev) => prev?.filter((p) => p.id !== postId));
    };

    return (
        <div class="container mx-auto px-4 py-8 max-w-2xl">
            <Title>Sosmed | Asepharyana</Title>

            <div class="mb-8">
                <h1 class="text-3xl font-bold gradient-text mb-2">Community Feed</h1>
                <p class="text-muted-foreground">Share your thoughts and connect with others.</p>
            </div>

            <Show when={user()} fallback={
                <div class="glass-card p-8 text-center rounded-2xl mb-8">
                    <p class="text-lg font-medium mb-4">Please sign in to post and interact.</p>
                    <a href="/login" class="px-6 py-2 bg-primary text-primary-foreground rounded-lg inline-block hover:bg-primary/90 transition-colors">
                        Sign In
                    </a>
                </div>
            }>
                <CreatePost onPostCreated={handlePostCreated} />
            </Show>

            <div class="space-y-6">
                <Show when={posts.loading}>
                    <div class="space-y-6">
                        <div class="glass-card h-40 rounded-2xl animate-pulse bg-muted/20" />
                        <div class="glass-card h-40 rounded-2xl animate-pulse bg-muted/20" />
                    </div>
                </Show>

                <For each={posts()}>
                    {(post) => (
                        <PostCard
                            post={post}
                            onPostUpdated={handlePostUpdated}
                            onDelete={handlePostDeleted}
                        />
                    )}
                </For>

                <Show when={posts()?.length === 0}>
                    <div class="text-center py-12 text-muted-foreground">
                        <p>No posts yet. Be the first to share something!</p>
                    </div>
                </Show>
            </div>
        </div>
    );
}
