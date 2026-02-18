/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: [
        './src/**/*.rs',
        './index.html',
    ],
    theme: {
        extend: {
            colors: {
                border: 'hsl(var(--border))',
                input: 'hsl(var(--input))',
                ring: 'hsl(var(--ring))',
                background: 'hsl(var(--background))',
                foreground: 'hsl(var(--foreground))',
                primary: {
                    DEFAULT: 'hsl(var(--primary))',
                    foreground: 'hsl(var(--primary-foreground))',
                },
                secondary: {
                    DEFAULT: 'hsl(var(--secondary))',
                    foreground: 'hsl(var(--secondary-foreground))',
                },
                destructive: {
                    DEFAULT: 'hsl(var(--destructive))',
                    foreground: 'hsl(var(--destructive-foreground))',
                },
                muted: {
                    DEFAULT: 'hsl(var(--muted))',
                    foreground: 'hsl(var(--muted-foreground))',
                },
                accent: {
                    DEFAULT: 'hsl(var(--accent))',
                    foreground: 'hsl(var(--accent-foreground))',
                },
                popover: {
                    DEFAULT: 'hsl(var(--popover))',
                    foreground: 'hsl(var(--popover-foreground))',
                },
                card: {
                    DEFAULT: 'hsl(var(--card))',
                    foreground: 'hsl(var(--card-foreground))',
                },
                neon: {
                    purple: 'hsl(var(--neon-purple))',
                    cyan: 'hsl(var(--neon-cyan))',
                    magenta: 'hsl(var(--neon-magenta))',
                },
            },
            borderRadius: {
                lg: 'var(--radius)',
                md: 'calc(var(--radius) - 2px)',
                sm: 'calc(var(--radius) - 4px)',
                xl: 'calc(var(--radius) + 4px)',
                '2xl': 'calc(var(--radius) + 8px)',
            },
            animation: {
                'fade-in': 'fadeIn 0.5s ease-out',
                'slide-up': 'slideUp 0.5s ease-out',
                'slide-down': 'slideDown 0.3s ease-out',
                'slide-in-right': 'slide-in-right 0.3s ease-out',
                'pulse-slow': 'pulse 3s ease-in-out infinite',
                'float': 'float 6s ease-in-out infinite',
                'float-slow': 'float-slow 8s ease-in-out infinite',
                'shimmer': 'shimmer 1.5s infinite',
                'gradient': 'gradient-shift 4s ease infinite',
                'bounce-in': 'bounce-in 0.5s ease-out',
                'glow': 'glow-pulse 2s ease-in-out infinite',
                'spin-slow': 'spin-slow 20s linear infinite',
                'spin-reverse': 'spin-reverse 30s linear infinite',
                'wave': 'wave 2s ease-in-out infinite',
                'morph': 'morph 8s ease-in-out infinite',
            },
            keyframes: {
                wave: {
                    '0%, 100%': { transform: 'rotate(0deg)' },
                    '25%': { transform: 'rotate(20deg)' },
                    '75%': { transform: 'rotate(-10deg)' },
                },
                'spin-reverse': {
                    from: { transform: 'rotate(360deg)' },
                    to: { transform: 'rotate(0deg)' },
                },
                fadeIn: {
                    '0%': { opacity: '0' },
                    '100%': { opacity: '1' },
                },
                slideUp: {
                    '0%': { transform: 'translateY(20px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                slideDown: {
                    '0%': { transform: 'translateY(-10px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                'slide-in-right': {
                    '0%': { transform: 'translateX(100%)', opacity: '0' },
                    '100%': { transform: 'translateX(0)', opacity: '1' },
                },
                float: {
                    '0%, 100%': { transform: 'translateY(0) rotate(0deg)' },
                    '25%': { transform: 'translateY(-10px) rotate(1deg)' },
                    '75%': { transform: 'translateY(5px) rotate(-1deg)' },
                },
                'float-slow': {
                    '0%, 100%': { transform: 'translateY(0) scale(1)' },
                    '50%': { transform: 'translateY(-20px) scale(1.03)' },
                },
                shimmer: {
                    '0%': { backgroundPosition: '-200% 0' },
                    '100%': { backgroundPosition: '200% 0' },
                },
                'gradient-shift': {
                    '0%, 100%': { backgroundPosition: '0% 50%' },
                    '50%': { backgroundPosition: '100% 50%' },
                },
                'bounce-in': {
                    '0%': { transform: 'scale(0.5)', opacity: '0' },
                    '50%': { transform: 'scale(1.1)' },
                    '100%': { transform: 'scale(1)', opacity: '1' },
                },
                'glow-pulse': {
                    '0%, 100%': { boxShadow: '0 0 20px hsla(var(--glow), 0.4), 0 0 40px hsla(var(--glow), 0.2)' },
                    '50%': { boxShadow: '0 0 30px hsla(var(--glow), 0.6), 0 0 60px hsla(var(--glow), 0.3)' },
                },
                morph: {
                    '0%, 100%': { borderRadius: '60% 40% 30% 70% / 60% 30% 70% 40%' },
                    '50%': { borderRadius: '30% 60% 70% 40% / 50% 60% 30% 60%' },
                },
                'spin-slow': {
                    '0%': { transform: 'rotate(0deg)' },
                    '100%': { transform: 'rotate(360deg)' },
                }
            },
            backgroundImage: {
                'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
                'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
            },
        },
    },
    plugins: [
        require('@kobalte/tailwindcss'),
    ],
}
