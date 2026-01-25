import { A } from '@solidjs/router';

export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer class="border-t border-white/10 mt-auto py-6 bg-background/50">
      <div class="container mx-auto px-4">
        <div class="flex flex-col sm:flex-row items-center justify-between gap-4 text-center sm:text-left">
          <A href="/" class="flex items-center gap-2 group">
            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-purple-600 flex items-center justify-center shadow-md group-hover:scale-105 transition-transform">
              <span class="text-white font-bold text-sm">A</span>
            </div>
            <span class="font-semibold text-foreground">Asepharyana</span>
          </A>
          <p class="text-xs text-muted-foreground">
            © {currentYear} Asepharyana. Made with ❤️ in Indonesia
          </p>
        </div>
      </div>
    </footer>
  );
}
