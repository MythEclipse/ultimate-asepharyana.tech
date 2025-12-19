import { createSignal, createContext, useContext, type ParentProps, onMount } from "solid-js";
import { httpClient } from "./http-client";
import type { User, LoginCredentials, RegisterData } from "../types/auth";

interface AuthContextType {
    user: () => User | null;
    loading: () => boolean;
    login: (credentials: LoginCredentials) => Promise<void>;
    register: (data: RegisterData) => Promise<void>;
    logout: () => Promise<void>;
    refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType>();

export function AuthProvider(props: ParentProps) {
    const [user, setUser] = createSignal<User | null>(null);
    const [loading, setLoading] = createSignal(true);

    const refreshUser = async () => {
        try {
            const data = await httpClient.fetchJson<{ user: User | null }>("/api/auth/verify");
            setUser(data.user ?? null);
        } catch (error) {
            console.error("Failed to verify session:", error);
            setUser(null);
        }
    };

    onMount(async () => {
        await refreshUser();
        setLoading(false);
    });

    const login = async (credentials: LoginCredentials) => {
        const data = await httpClient.request<{ user: User }>("/api/auth/login", "POST", credentials);
        setUser(data.user);
    };

    const register = async (data: RegisterData) => {
        await httpClient.request("/api/auth/register", "POST", data);
        // After registration, login automatically
        await login({ email: data.email, password: data.password });
    };

    const logout = async () => {
        try {
            await httpClient.request("/api/auth/logout", "POST");
        } catch (error) {
            console.error("Logout error:", error);
        } finally {
            setUser(null);
        }
    };

    return (
        <AuthContext.Provider
            value={{
                user,
                loading,
                login,
                register,
                logout,
                refreshUser,
            }}
        >
            {props.children}
        </AuthContext.Provider>
    );
}

export function useAuth() {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
}
