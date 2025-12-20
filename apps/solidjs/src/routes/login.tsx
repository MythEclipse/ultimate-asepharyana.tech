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
    const [showPassword, setShowPassword] = createSignal(false);

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
                    <div class="w-12 h-12 rounded-full border-2 border-primary border-t-transparent animate-spin" />
                </div>
            </Show>

            <Show when={!loading() && !user()}>
                <div class="min-h-screen flex items-center justify-center p-4 relative overflow-hidden">
                    {/* Animated Background */}
                    <div class="absolute inset-0 -z-10">
                        <div class="gradient-orb gradient-orb-1 w-[400px] h-[400px] left-[-10%] top-[20%]" />
                        <div class="gradient-orb gradient-orb-2 w-[500px] h-[500px] right-[-15%] bottom-[10%]" />
                    </div>

                    <Motion.div
                        initial={{ opacity: 0, y: 30, scale: 0.95 }}
                        animate={{ opacity: 1, y: 0, scale: 1 }}
                        transition={{ duration: 0.5 }}
                        class="w-full max-w-md"
                    >
                        <div class="glass-card rounded-2xl overflow-hidden">
                            <div class="p-8">
                                {/* Header */}
                                <div class="text-center mb-8">
                                    <Motion.div
                                        initial={{ opacity: 0, scale: 0.5 }}
                                        animate={{ opacity: 1, scale: 1 }}
                                        transition={{ duration: 0.4, delay: 0.1 }}
                                        class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-gradient-to-br from-primary via-accent to-neon-cyan flex items-center justify-center shadow-lg shadow-primary/25"
                                    >
                                        <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                        </svg>
                                    </Motion.div>
                                    <h1 class="text-2xl font-bold gradient-text">
                                        Welcome Back
                                    </h1>
                                    <p class="text-muted-foreground mt-1">
                                        Sign in to your account
                                    </p>
                                </div>

                                {/* Error Message */}
                                <Show when={error()}>
                                    <Motion.div
                                        initial={{ opacity: 0, x: -20 }}
                                        animate={{ opacity: 1, x: 0 }}
                                        class="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-xl flex items-start gap-3"
                                    >
                                        <svg class="h-5 w-5 text-destructive mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <p class="text-sm text-destructive">{error()}</p>
                                    </Motion.div>
                                </Show>

                                {/* Form */}
                                <form onSubmit={handleSubmit} class="space-y-5">
                                    {/* Email */}
                                    <div class="space-y-2">
                                        <label for="email" class="text-sm font-medium text-foreground">
                                            Email
                                        </label>
                                        <div class="relative group">
                                            <svg class="absolute left-4 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground group-focus-within:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                                            </svg>
                                            <input
                                                id="email"
                                                name="email"
                                                type="email"
                                                required
                                                value={formData().email}
                                                onInput={handleChange}
                                                class="w-full pl-12 pr-4 py-3.5 rounded-xl border border-border bg-background/50 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all placeholder:text-muted-foreground/60"
                                                placeholder="your@email.com"
                                                disabled={isLoading()}
                                            />
                                        </div>
                                    </div>

                                    {/* Password */}
                                    <div class="space-y-2">
                                        <label for="password" class="text-sm font-medium text-foreground">
                                            Password
                                        </label>
                                        <div class="relative group">
                                            <svg class="absolute left-4 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground group-focus-within:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                            </svg>
                                            <input
                                                id="password"
                                                name="password"
                                                type={showPassword() ? "text" : "password"}
                                                required
                                                value={formData().password}
                                                onInput={handleChange}
                                                class="w-full pl-12 pr-12 py-3.5 rounded-xl border border-border bg-background/50 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all placeholder:text-muted-foreground/60"
                                                placeholder="••••••••"
                                                disabled={isLoading()}
                                            />
                                            <button
                                                type="button"
                                                onClick={() => setShowPassword(!showPassword())}
                                                class="absolute right-4 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                                            >
                                                <Show when={showPassword()} fallback={
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                                    </svg>
                                                }>
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                                                    </svg>
                                                </Show>
                                            </button>
                                        </div>
                                    </div>

                                    {/* Submit */}
                                    <button
                                        type="submit"
                                        disabled={isLoading()}
                                        class="w-full py-3.5 text-base font-semibold rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 transition-all disabled:opacity-50 flex items-center justify-center gap-2 shadow-lg shadow-primary/25 hover:shadow-xl hover:shadow-primary/30"
                                    >
                                        <Show when={isLoading()} fallback={
                                            <>
                                                Sign In
                                                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                                </svg>
                                            </>
                                        }>
                                            <div class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                                            Signing in...
                                        </Show>
                                    </button>
                                </form>

                                {/* Sign up link */}
                                <div class="mt-6 text-center">
                                    <p class="text-sm text-muted-foreground">
                                        Don't have an account?{" "}
                                        <A href="/register" class="text-primary hover:underline font-semibold">
                                            Sign up
                                        </A>
                                    </p>
                                </div>
                            </div>

                            {/* Footer */}
                            <div class="px-8 py-4 bg-muted/30 border-t border-border/50">
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

