use leptos::*;
use crate::components::navbar::Navbar;
use crate::components::ui::navigation_progress::NavigationProgress;
use crate::providers::{provide_theme, provide_auth};

#[component]
pub fn ClientLayout(children: Children) -> impl IntoView {
    // Provide contexts at the layout level
    provide_theme();
    provide_auth();

    view! {
        <div class="min-h-screen flex flex-col relative overflow-x-hidden bg-background text-foreground transition-colors duration-700">
            // Premium Cinematic Infrastructure (Global Background)
            <div class="fixed inset-0 pointer-events-none -z-50 select-none overflow-hidden">
                <div class="absolute inset-0 bg-gradient-to-br from-indigo-500/5 via-transparent to-purple-500/5 opacity-50" />
                
                // Pulsing Stellar Orbs
                <div class="absolute top-[-10%] left-[-5%] w-[60rem] h-[60rem] bg-indigo-600/10 rounded-full blur-[120px] animate-tilt-slow opacity-60" />
                <div class="absolute bottom-[-15%] right-[-10%] w-[70rem] h-[70rem] bg-purple-600/10 rounded-full blur-[150px] animate-tilt-reverse-slow opacity-40" />
                
                // Central Ambient Glow
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[50rem] h-[50rem] bg-blue-500/5 rounded-full blur-[180px] opacity-20" />
                
                // Static Noise layer for texture depth
                <div class="absolute inset-0 opacity-[0.03] mix-blend-overlay pointer-events-none bg-[url('https://grainy-gradients.vercel.app/noise.svg')]" />
            </div>

            <NavigationProgress/>
            <Navbar/>
            
            <main class="relative z-10 flex-1 flex flex-col max-w-[100vw]">
                {children()}
            </main>
        </div>
    }
}
