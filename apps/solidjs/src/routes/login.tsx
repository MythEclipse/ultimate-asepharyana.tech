import { Title } from "@solidjs/meta";
import { A, useNavigate } from "@solidjs/router";
import { createSignal, createEffect, Show } from "solid-js";
import { Motion } from "solid-motionone";
import { useAuth } from "~/lib/auth-context";

export default function LoginPage() {
    const navigate = useNavigate();
    const { login, user, loading } = useAuth();

    const [formData, setFormData] = createSignal({
        email: "",
        password: "",
    });
    const [error, setError] = createSignal("");
    const [isLoading, setIsLoading] = createSignal(false);

    // Redirect if already logged in
    createEffect(() => {
        if (user() && !loading()) {
            navigate("/dashboard");
        }
    });

    const handleChange = (e: Event) => {
        const target = e.target as HTMLInputElement;
        setFormData((prev) => ({
            ...prev,
            [target.name]: target.value,
        }));
        setError("");
    };

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        setError("");
        setIsLoading(true);

        try {
            await login(formData());
            navigate("/dashboard");
        } catch (err) {
            setError(err instanceof Error ? err.message : "Login failed. Please check your credentials.");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <Title>Login | Asepharyana</Title>
            <Show when={loading()}>
                <div class="min-h-screen flex items-center justify-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary" />
                </div>
            </Show>

            <Show when={!loading() && !user()}>
                <div class="min-h-screen flex items-center justify-center p-4 bg-gradient-to-br from-background via-background to-muted">
                    <Motion.div
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5 }}
                        class="w-full max-w-md"
                    >
                        <div class="bg-card rounded-2xl shadow-2xl border border-border overflow-hidden">
                            <div class="p-8">
                                <div class="text-center mb-8">
                                    <h1 class="text-3xl font-bold bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent">
                                        Welcome Back
                                    </h1>
                                    <p class="text-muted-foreground mt-2">
                                        Sign in to your account
                                    </p>
                                </div>

                                <Show when={error()}>
                                    <Motion.div
                                        initial={{ opacity: 0, x: -20 }}
                                        animate={{ opacity: 1, x: 0 }}
                                        class="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg flex items-start gap-3"
                                    >
                                        <svg class="h-5 w-5 text-destructive mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <circle cx="12" cy="12" r="10" stroke-width="2" />
                                            <line x1="12" y1="8" x2="12" y2="12" stroke-width="2" />
                                            <circle cx="12" cy="16" r="1" fill="currentColor" />
                                        </svg>
                                        <p class="text-sm text-destructive">{error()}</p>
                                    </Motion.div>
                                </Show>

                                <form onSubmit={handleSubmit} class="space-y-6">
                                    <div class="space-y-2">
                                        <label for="email" class="text-sm font-medium text-foreground">
                                            Email
                                        </label>
                                        <div class="relative">
                                            <svg class="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                                            </svg>
                                            <input
                                                id="email"
                                                name="email"
                                                type="email"
                                                required
                                                value={formData().email}
                                                onInput={handleChange}
                                                class="w-full pl-10 pr-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all"
                                                placeholder="your@email.com"
                                                disabled={isLoading()}
                                            />
                                        </div>
                                    </div>

                                    <div class="space-y-2">
                                        <label for="password" class="text-sm font-medium text-foreground">
                                            Password
                                        </label>
                                        <div class="relative">
                                            <svg class="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                            </svg>
                                            <input
                                                id="password"
                                                name="password"
                                                type="password"
                                                required
                                                value={formData().password}
                                                onInput={handleChange}
                                                class="w-full pl-10 pr-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all"
                                                placeholder="••••••••"
                                                disabled={isLoading()}
                                            />
                                        </div>
                                    </div>

                                    <button
                                        type="submit"
                                        disabled={isLoading()}
                                        class="w-full py-3 text-base font-semibold rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all disabled:opacity-50 flex items-center justify-center gap-2"
                                    >
                                        {isLoading() ? "Signing in..." : (
                                            <>
                                                Sign In
                                                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                                </svg>
                                            </>
                                        )}
                                    </button>
                                </form>

                                <div class="mt-6 text-center">
                                    <p class="text-sm text-muted-foreground">
                                        Don't have an account?{" "}
                                        <A href="/register" class="text-primary hover:underline font-semibold">
                                            Sign up
                                        </A>
                                    </p>
                                </div>
                            </div>

                            <div class="px-8 py-4 bg-muted/50 border-t border-border">
                                <p class="text-xs text-center text-muted-foreground">
                                    By signing in, you agree to our Terms of Service and Privacy Policy
                                </p>
                            </div>
                        </div>
                    </Motion.div>
                </div>
            </Show>
        </>
    );
}
