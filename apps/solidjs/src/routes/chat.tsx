import { Title } from "@solidjs/meta";
import { createSignal, createEffect, For, Show, onMount, onCleanup } from "solid-js";
import { httpClient } from "~/lib/http-client";

interface Message {
    role: "user" | "assistant";
    content: string;
}

export default function ChatPage() {
    const [messages, setMessages] = createSignal<Message[]>([]);
    const [input, setInput] = createSignal("");
    const [isLoading, setIsLoading] = createSignal(false);
    let messagesEndRef: HTMLDivElement | undefined;

    const scrollToBottom = () => {
        messagesEndRef?.scrollIntoView({ behavior: "smooth" });
    };

    createEffect(() => {
        if (messages().length > 0) {
            scrollToBottom();
        }
    });

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        const userMessage = input().trim();
        if (!userMessage || isLoading()) return;

        setMessages((prev) => [...prev, { role: "user", content: userMessage }]);
        setInput("");
        setIsLoading(true);

        try {
            const response = await httpClient.request<{ content: string }>(
                "/api/ai/chat",
                "POST",
                { message: userMessage, history: messages() }
            );
            setMessages((prev) => [...prev, { role: "assistant", content: response.content }]);
        } catch (error) {
            setMessages((prev) => [
                ...prev,
                { role: "assistant", content: "Sorry, I encountered an error. Please try again." }
            ]);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <Title>AI Chat | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground flex flex-col">
                {/* Header */}
                <div class="border-b border-border p-4">
                    <div class="max-w-4xl mx-auto">
                        <h1 class="text-2xl font-bold bg-gradient-to-r from-green-600 to-teal-600 bg-clip-text text-transparent">
                            🤖 AI Chat
                        </h1>
                        <p class="text-sm text-muted-foreground">Chat with AI powered by multiple models</p>
                    </div>
                </div>

                {/* Messages */}
                <div class="flex-1 overflow-y-auto p-4">
                    <div class="max-w-4xl mx-auto space-y-4">
                        <Show when={messages().length === 0}>
                            <div class="text-center py-20">
                                <span class="text-6xl mb-4 block">💬</span>
                                <h2 class="text-xl font-semibold mb-2">Start a conversation</h2>
                                <p class="text-muted-foreground">Ask me anything! I can help with coding, writing, analysis, and more.</p>
                            </div>
                        </Show>

                        <For each={messages()}>
                            {(message) => (
                                <div class={`flex ${message.role === "user" ? "justify-end" : "justify-start"}`}>
                                    <div
                                        class={`max-w-[80%] p-4 rounded-2xl ${message.role === "user"
                                                ? "bg-primary text-primary-foreground"
                                                : "bg-card border border-border"
                                            }`}
                                    >
                                        <p class="whitespace-pre-wrap">{message.content}</p>
                                    </div>
                                </div>
                            )}
                        </For>

                        <Show when={isLoading()}>
                            <div class="flex justify-start">
                                <div class="bg-card border border-border p-4 rounded-2xl">
                                    <div class="flex gap-1">
                                        <div class="w-2 h-2 rounded-full bg-muted-foreground animate-bounce" style="animation-delay: 0ms" />
                                        <div class="w-2 h-2 rounded-full bg-muted-foreground animate-bounce" style="animation-delay: 150ms" />
                                        <div class="w-2 h-2 rounded-full bg-muted-foreground animate-bounce" style="animation-delay: 300ms" />
                                    </div>
                                </div>
                            </div>
                        </Show>

                        <div ref={messagesEndRef} />
                    </div>
                </div>

                {/* Input */}
                <div class="border-t border-border p-4">
                    <form onSubmit={handleSubmit} class="max-w-4xl mx-auto flex gap-3">
                        <input
                            type="text"
                            value={input()}
                            onInput={(e) => setInput(e.target.value)}
                            placeholder="Type your message..."
                            class="flex-1 px-4 py-3 rounded-xl border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary"
                            disabled={isLoading()}
                        />
                        <button
                            type="submit"
                            disabled={isLoading() || !input().trim()}
                            class="px-6 py-3 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 transition-all"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                            </svg>
                        </button>
                    </form>
                </div>
            </main>
        </>
    );
}
