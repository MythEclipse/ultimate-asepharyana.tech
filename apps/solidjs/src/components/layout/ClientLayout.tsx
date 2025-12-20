import type { ParentProps } from "solid-js";
import { ThemeProvider } from "../providers/theme-provider";
import { AuthProvider } from "~/lib/auth-context";
import { Navbar } from "../navbar/Navbar";
import { Footer } from "./Footer";
import { Toaster } from "solid-toast";

export function ClientLayout(props: ParentProps) {
    return (
        <ThemeProvider>
            <AuthProvider>
                <div class="min-h-screen flex flex-col">
                    <Navbar />
                    <main class="flex-1">{props.children}</main>
                    <Footer />
                </div>
                <Toaster position="bottom-right" />
            </AuthProvider>
        </ThemeProvider>
    );
}

