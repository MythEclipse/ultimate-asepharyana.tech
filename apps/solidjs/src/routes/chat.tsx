import { Title } from "@solidjs/meta";
import { createSignal, createEffect, For, Show } from "solid-js";
import { Motion } from "solid-motionone";
import toast from "solid-toast";
import { httpClient } from "~/lib/http-client";

interface Message {
    role: "user" | "assistant";
    content: string;
    id: string;
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

    const generateId = () => Math.random().toString(36).substring(7);

    const copyToClipboard = async (content: string) => {
        try {
            await navigator.clipboard.writeText(content);
            toast.success("Copied to clipboard!");
        } catch {
            toast.error("Failed to copy");
        }
    };

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        const userMessage = input().trim();
        if (!userMessage || isLoading()) return;

        const newUserMessage: Message = { role: "user", content: userMessage, id: generateId() };
        setMessages((prev) => [...prev, newUserMessage]);
        setInput("");
        setIsLoading(true);

        try {
            const response = await httpClient.request<{ content: string }>(
                "/api/ai/chat",
                "POST",
                { message: userMessage, history: messages() }
            );
            const newAssistantMessage: Message = { role: "assistant", content: response.content, id: generateId() };
            setMessages((prev) => [...prev, newAssistantMessage]);
        } catch (error) {
            const errorMessage: Message = {
                role: "assistant",
                content: "Sorry, I encountered an error. Please try again.",
                id: generateId()
            };
            setMessages((prev) => [...prev, errorMessage]);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <Title>AI Chat | Asepharyana</Title>
            <main class="min-h-screen bg-background text-foreground flex flex-col">
                {/* Header */}
                <div class="glass-subtle border-b border-white/10 p-4">
                    <div class="max-w-4xl mx-auto flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-green-500 via-teal-500 to-cyan-500 flex items-center justify-center shadow-lg">
                            <span class="text-xl">🤖</span>
                        </div>
                        <div>
                            <h1 class="text-xl font-bold gradient-text">AI Chat</h1>
                            <p class="text-xs text-muted-foreground">Powered by multiple AI models</p>
                        </div>
                    </div>
                </div>

                {/* Messages */}
                <div class="flex-1 overflow-y-auto p-4">
                    <div class="max-w-4xl mx-auto space-y-4">
                        <Show when={messages().length === 0}>
                            <Motion.div
                                initial={{ opacity: 0, y: 20 }}
                                animate={{ opacity: 1, y: 0 }}
                                class="text-center py-20"
                            >
                                <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-green-500/20 via-teal-500/20 to-cyan-500/20 flex items-center justify-center">
                                    <span class="text-5xl">💬</span>
                                </div>
                                <h2 class="text-2xl font-bold mb-3 gradient-text">Start a conversation</h2>
                                <p class="text-muted-foreground max-w-md mx-auto">
                                    Ask me anything! I can help with coding, writing, analysis, and more.
                                </p>
                                <div class="mt-8 flex flex-wrap justify-center gap-2">
                                    {["Explain recursion", "Write a poem", "Debug my code"].map((suggestion) => (
                                        <button
                                            onClick={() => setInput(suggestion)}
                                            class="px-4 py-2 rounded-xl glass-subtle hover:bg-white/10 text-sm transition-colors"
                                        >
                                            {suggestion}
                                        </button>
                                    ))}
                                </div>
                            </Motion.div>
                        </Show>

                        <For each={messages()}>
                            {(message, index) => (
                                <Motion.div
                                    initial={{ opacity: 0, y: 10, scale: 0.98 }}
                                    animate={{ opacity: 1, y: 0, scale: 1 }}
                                    transition={{ duration: 0.3, delay: index() * 0.05 }}
                                    class={`flex ${message.role === "user" ? "justify-end" : "justify-start"}`}
                                >
                                    <div class="group relative">
                                        <div
                                            class={`max-w-[85%] md:max-w-[75%] p-4 rounded-2xl ${message.role === "user"
                                                    ? "bg-primary text-primary-foreground rounded-br-md"
                                                    : "glass-card rounded-bl-md"
                                                }`}
                                        >
                                            <p class="whitespace-pre-wrap text-[15px] leading-relaxed">{message.content}</p>
                                        </div>

                                        {/* Copy button for assistant messages */}
                                        <Show when={message.role === "assistant"}>
                                            <button
                                                onClick={() => copyToClipboard(message.content)}
                                                class="absolute -right-2 -top-2 opacity-0 group-hover:opacity-100 p-2 rounded-lg glass-subtle hover:bg-white/20 transition-all"
                                                title="Copy to clipboard"
                                            >
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                                                </svg>
                                            </button>
                                        </Show>
                                    </div>
                                </Motion.div>
                            )}
                        </For>

                        {/* Typing Indicator */}
                        <Show when={isLoading()}>
                            <Motion.div
                                initial={{ opacity: 0, y: 10 }}
                                animate={{ opacity: 1, y: 0 }}
                                class="flex justify-start"
                            >
                                <div class="glass-card p-4 rounded-2xl rounded-bl-md">
                                    <div class="flex gap-1.5">
                                        <div class="w-2.5 h-2.5 rounded-full bg-primary animate-bounce" style="animation-delay: 0ms" />
                                        <div class="w-2.5 h-2.5 rounded-full bg-accent animate-bounce" style="animation-delay: 150ms" />
                                        <div class="w-2.5 h-2.5 rounded-full bg-neon-cyan animate-bounce" style="animation-delay: 300ms" />
                                    </div>
                                </div>
                            </Motion.div>
                        </Show>

                        <div ref={messagesEndRef} />
                    </div>
                </div>

                {/* Input */}
                <div class="glass-subtle border-t border-white/10 p-4">
                    <form onSubmit={handleSubmit} class="max-w-4xl mx-auto flex gap-3">
                        <input
                            type="text"
                            value={input()}
                            onInput={(e) => setInput(e.target.value)}
                            placeholder="Type your message..."
                            class="flex-1 px-5 py-3.5 rounded-xl border border-border bg-background/50 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all placeholder:text-muted-foreground/60"
                            disabled={isLoading()}
                        />
                        <button
                            type="submit"
                            disabled={isLoading() || !input().trim()}
                            class="px-5 py-3.5 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 transition-all shadow-lg shadow-primary/25 hover:shadow-xl hover:shadow-primary/30 disabled:shadow-none"
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

