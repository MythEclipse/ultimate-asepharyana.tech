import { Title } from '@solidjs/meta';
import { A, useNavigate } from '@solidjs/router';
import { createSignal, createEffect, Show } from 'solid-js';
import { Motion } from 'solid-motionone';
import { useAuth } from '~/lib/auth-context';

export default function RegisterPage() {
  const navigate = useNavigate();
  const { register, user, loading } = useAuth();

  const [formData, setFormData] = createSignal({
    name: '',
    email: '',
    password: '',
    confirmPassword: '',
  });
  const [error, setError] = createSignal('');
  const [isLoading, setIsLoading] = createSignal(false);
  const [showPassword, setShowPassword] = createSignal(false);
  const [showConfirmPassword, setShowConfirmPassword] = createSignal(false);

  createEffect(() => {
    if (user() && !loading()) {
      navigate('/dashboard');
    }
  });

  const handleChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setFormData((prev) => ({
      ...prev,
      [target.name]: target.value,
    }));
    setError('');
  };

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setError('');

    const data = formData();
    if (data.password !== data.confirmPassword) {
      setError('Passwords do not match');
      return;
    }

    if (data.password.length < 6) {
      setError('Password must be at least 6 characters');
      return;
    }

    setIsLoading(true);

    try {
      await register({
        name: data.name,
        email: data.email,
        password: data.password,
      });
      navigate('/dashboard');
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : 'Registration failed. Please try again.',
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <>
      <Title>Register | Asepharyana</Title>
      <Show when={loading()}>
        <div class="min-h-screen flex items-center justify-center">
          <div class="relative">
            <div class="w-16 h-16 rounded-full border-4 border-primary/20 border-t-primary animate-spin" />
          </div>
        </div>
      </Show>

      <Show when={!loading() && !user()}>
        <div class="min-h-screen flex items-center justify-center p-4 relative overflow-hidden">
          {/* Animated background orbs */}
          <div class="fixed inset-0 overflow-hidden pointer-events-none">
            <div class="absolute top-1/4 -right-20 w-96 h-96 bg-gradient-to-br from-primary/30 to-purple-500/30 rounded-full blur-3xl animate-float" />
            <div
              class="absolute bottom-1/4 -left-20 w-96 h-96 bg-gradient-to-tr from-cyan-500/30 to-blue-500/30 rounded-full blur-3xl animate-float"
              style={{ 'animation-delay': '-4s' }}
            />
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-72 h-72 bg-gradient-to-r from-pink-500/20 to-orange-500/20 rounded-full blur-3xl animate-pulse-slow" />
          </div>

          <Motion.div
            initial={{ opacity: 0, y: 30, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            transition={{ duration: 0.6, easing: 'ease-out' }}
            class="w-full max-w-md relative z-10"
          >
            <div class="glass-card rounded-3xl overflow-hidden shadow-2xl">
              <div class="p-8">
                {/* Header */}
                <div class="text-center mb-8">
                  <Motion.div
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    transition={{ duration: 0.5, delay: 0.2 }}
                    class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-gradient-to-br from-primary to-purple-600 flex items-center justify-center shadow-lg shadow-primary/30"
                  >
                    <svg
                      class="w-8 h-8 text-white"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
                      />
                    </svg>
                  </Motion.div>
                  <h1 class="text-3xl font-bold gradient-text">
                    Create Account
                  </h1>
                  <p class="text-muted-foreground mt-2">
                    Join us and start your journey
                  </p>
                </div>

                {/* Error message */}
                <Show when={error()}>
                  <Motion.div
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    class="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-xl flex items-start gap-3"
                  >
                    <svg
                      class="h-5 w-5 text-destructive mt-0.5 flex-shrink-0"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                    <p class="text-sm text-destructive">{error()}</p>
                  </Motion.div>
                </Show>

                <form onSubmit={handleSubmit} class="space-y-5">
                  {/* Name */}
                  <div class="space-y-2">
                    <label
                      for="name"
                      class="text-sm font-medium text-foreground"
                    >
                      Name
                    </label>
                    <div class="relative group">
                      <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <svg
                          class="h-5 w-5 text-muted-foreground group-focus-within:text-primary transition-colors"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                          />
                        </svg>
                      </div>
                      <input
                        id="name"
                        name="name"
                        type="text"
                        required
                        value={formData().name}
                        onInput={handleChange}
                        class="w-full pl-12 pr-4 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all placeholder:text-muted-foreground/50"
                        placeholder="Your name"
                        disabled={isLoading()}
                      />
                    </div>
                  </div>

                  {/* Email */}
                  <div class="space-y-2">
                    <label
                      for="email"
                      class="text-sm font-medium text-foreground"
                    >
                      Email
                    </label>
                    <div class="relative group">
                      <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <svg
                          class="h-5 w-5 text-muted-foreground group-focus-within:text-primary transition-colors"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                          />
                        </svg>
                      </div>
                      <input
                        id="email"
                        name="email"
                        type="email"
                        required
                        value={formData().email}
                        onInput={handleChange}
                        class="w-full pl-12 pr-4 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all placeholder:text-muted-foreground/50"
                        placeholder="your@email.com"
                        disabled={isLoading()}
                      />
                    </div>
                  </div>

                  {/* Password */}
                  <div class="space-y-2">
                    <label
                      for="password"
                      class="text-sm font-medium text-foreground"
                    >
                      Password
                    </label>
                    <div class="relative group">
                      <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <svg
                          class="h-5 w-5 text-muted-foreground group-focus-within:text-primary transition-colors"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                          />
                        </svg>
                      </div>
                      <input
                        id="password"
                        name="password"
                        type={showPassword() ? 'text' : 'password'}
                        required
                        value={formData().password}
                        onInput={handleChange}
                        class="w-full pl-12 pr-12 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all placeholder:text-muted-foreground/50"
                        placeholder="••••••••"
                        disabled={isLoading()}
                      />
                      <button
                        type="button"
                        onClick={() => setShowPassword(!showPassword())}
                        class="absolute inset-y-0 right-0 pr-4 flex items-center text-muted-foreground hover:text-foreground transition-colors"
                      >
                        <Show
                          when={showPassword()}
                          fallback={
                            <svg
                              class="h-5 w-5"
                              fill="none"
                              stroke="currentColor"
                              viewBox="0 0 24 24"
                            >
                              <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                              />
                              <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                              />
                            </svg>
                          }
                        >
                          <svg
                            class="h-5 w-5"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
                            />
                          </svg>
                        </Show>
                      </button>
                    </div>
                  </div>

                  {/* Confirm Password */}
                  <div class="space-y-2">
                    <label
                      for="confirmPassword"
                      class="text-sm font-medium text-foreground"
                    >
                      Confirm Password
                    </label>
                    <div class="relative group">
                      <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                        <svg
                          class="h-5 w-5 text-muted-foreground group-focus-within:text-primary transition-colors"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                          />
                        </svg>
                      </div>
                      <input
                        id="confirmPassword"
                        name="confirmPassword"
                        type={showConfirmPassword() ? 'text' : 'password'}
                        required
                        value={formData().confirmPassword}
                        onInput={handleChange}
                        class="w-full pl-12 pr-12 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all placeholder:text-muted-foreground/50"
                        placeholder="••••••••"
                        disabled={isLoading()}
                      />
                      <button
                        type="button"
                        onClick={() =>
                          setShowConfirmPassword(!showConfirmPassword())
                        }
                        class="absolute inset-y-0 right-0 pr-4 flex items-center text-muted-foreground hover:text-foreground transition-colors"
                      >
                        <Show
                          when={showConfirmPassword()}
                          fallback={
                            <svg
                              class="h-5 w-5"
                              fill="none"
                              stroke="currentColor"
                              viewBox="0 0 24 24"
                            >
                              <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                              />
                              <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                              />
                            </svg>
                          }
                        >
                          <svg
                            class="h-5 w-5"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
                            />
                          </svg>
                        </Show>
                      </button>
                    </div>
                  </div>

                  <button
                    type="submit"
                    disabled={isLoading()}
                    class="w-full py-4 text-base font-semibold rounded-xl bg-gradient-to-r from-primary via-purple-600 to-primary text-white hover:opacity-90 transition-all disabled:opacity-50 flex items-center justify-center gap-2 mt-6 shadow-lg shadow-primary/30 hover:shadow-xl hover:shadow-primary/40 bg-[length:200%_100%] hover:bg-right"
                    style={{
                      'background-size': '200% 100%',
                      transition: 'all 0.5s ease',
                    }}
                  >
                    {isLoading() ? (
                      <>
                        <div class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                        Creating account...
                      </>
                    ) : (
                      <>
                        Create Account
                        <svg
                          class="h-5 w-5"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M14 5l7 7m0 0l-7 7m7-7H3"
                          />
                        </svg>
                      </>
                    )}
                  </button>
                </form>
              </div>

              {/* Footer */}
              <div class="px-8 py-6 bg-muted/30 border-t border-border/50 text-center">
                <p class="text-sm text-muted-foreground">
                  Already have an account?{' '}
                  <A
                    href="/login"
                    class="text-primary hover:underline font-semibold"
                  >
                    Sign in
                  </A>
                </p>
              </div>
            </div>
          </Motion.div>
        </div>
      </Show>
    </>
  );
}
