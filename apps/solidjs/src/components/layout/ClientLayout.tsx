import type { ParentProps } from "solid-js";
import { ThemeProvider } from "../providers/theme-provider";
import { AuthProvider } from "~/lib/auth-context";
import { Navbar } from "../navbar/Navbar";
import { Toaster } from "solid-toast";

export function ClientLayout(props: ParentProps) {
    return (
        <ThemeProvider>
            <AuthProvider>
                <Navbar />
                <main>{props.children}</main>
                <Toaster position="bottom-right" />
            </AuthProvider>
        </ThemeProvider>
    );
}
