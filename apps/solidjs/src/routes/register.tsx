import { Title } from "@solidjs/meta";
import { A, useNavigate } from "@solidjs/router";
import { createSignal, createEffect, Show } from "solid-js";
import { Motion } from "solid-motionone";
import { useAuth } from "~/lib/auth-context";

export default function RegisterPage() {
    const navigate = useNavigate();
    const { register, user, loading } = useAuth();

    const [formData, setFormData] = createSignal({
        name: "",
        email: "",
        password: "",
        confirmPassword: "",
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

        const data = formData();
        if (data.password !== data.confirmPassword) {
            setError("Passwords do not match");
            return;
        }

        if (data.password.length < 6) {
            setError("Password must be at least 6 characters");
            return;
        }

        setIsLoading(true);

        try {
            await register({
                name: data.name,
                email: data.email,
                password: data.password,
            });
            navigate("/dashboard");
        } catch (err) {
            setError(err instanceof Error ? err.message : "Registration failed. Please try again.");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <Title>Register | Asepharyana</Title>
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
                                        Create Account
                                    </h1>
                                    <p class="text-muted-foreground mt-2">
                                        Sign up for a new account
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

                                <form onSubmit={handleSubmit} class="space-y-4">
                                    <div class="space-y-2">
                                        <label for="name" class="text-sm font-medium text-foreground">Name</label>
                                        <input
                                            id="name"
                                            name="name"
                                            type="text"
                                            required
                                            value={formData().name}
                                            onInput={handleChange}
                                            class="w-full px-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all"
                                            placeholder="Your name"
                                            disabled={isLoading()}
                                        />
                                    </div>

                                    <div class="space-y-2">
                                        <label for="email" class="text-sm font-medium text-foreground">Email</label>
                                        <input
                                            id="email"
                                            name="email"
                                            type="email"
                                            required
                                            value={formData().email}
                                            onInput={handleChange}
                                            class="w-full px-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all"
                                            placeholder="your@email.com"
                                            disabled={isLoading()}
                                        />
                                    </div>

                                    <div class="space-y-2">
                                        <label for="password" class="text-sm font-medium text-foreground">Password</label>
                                        <input
                                            id="password"
                                            name="password"
                                            type="password"
                                            required
                                            value={formData().password}
                                            onInput={handleChange}
                                            class="w-full px-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all"
                                            placeholder="••••••••"
                                            disabled={isLoading()}
                                        />
                                    </div>

                                    <div class="space-y-2">
                                        <label for="confirmPassword" class="text-sm font-medium text-foreground">Confirm Password</label>
                                        <input
                                            id="confirmPassword"
                                            name="confirmPassword"
                                            type="password"
                                            required
                                            value={formData().confirmPassword}
                                            onInput={handleChange}
                                            class="w-full px-4 py-3 rounded-lg border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all"
                                            placeholder="••••••••"
                                            disabled={isLoading()}
                                        />
                                    </div>

                                    <button
                                        type="submit"
                                        disabled={isLoading()}
                                        class="w-full py-3 text-base font-semibold rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all disabled:opacity-50 flex items-center justify-center gap-2 mt-6"
                                    >
                                        {isLoading() ? "Creating account..." : (
                                            <>
                                                Create Account
                                                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                                </svg>
                                            </>
                                        )}
                                    </button>
                                </form>

                                <div class="mt-6 text-center">
                                    <p class="text-sm text-muted-foreground">
                                        Already have an account?{" "}
                                        <A href="/login" class="text-primary hover:underline font-semibold">
                                            Sign in
                                        </A>
                                    </p>
                                </div>
                            </div>
                        </div>
                    </Motion.div>
                </div>
            </Show>
        </>
    );
}
