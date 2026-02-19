/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: [
        './src/**/*.rs',
        './index.html',
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Outfit', 'sans-serif'],
                display: ['Space Grotesk', 'sans-serif'],
            },
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
                    blue: '#00d4ff',
                    lime: '#39ff14',
                },
            },
            borderRadius: {
                lg: 'var(--radius)',
                md: 'calc(var(--radius) - 2px)',
                sm: 'calc(var(--radius) - 4px)',
                xl: 'calc(var(--radius) + 4px)',
                '2xl': 'calc(var(--radius) + 8px)',
                '3xl': 'calc(var(--radius) + 16px)',
                '4xl': 'calc(var(--radius) + 24px)',
            },
            animation: {
                'fade-in': 'fadeIn 0.7s cubic-bezier(0.16, 1, 0.3, 1)',
                'slide-up': 'slideUp 0.7s cubic-bezier(0.16, 1, 0.3, 1)',
                'slide-down': 'slideDown 0.4s ease-out',
                'slide-in-right': 'slide-in-right 0.5s ease-out',
                'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                'float': 'float 6s ease-in-out infinite',
                'float-slow': 'float-slow 10s ease-in-out infinite',
                'shimmer': 'shimmer 2s infinite linear',
                'gradient': 'gradient-shift 8s ease infinite',
                'bounce-in': 'bounce-in 0.6s cubic-bezier(0.34, 1.56, 0.64, 1)',
                'glow': 'glow-pulse 3s ease-in-out infinite',
                'spin-slow': 'spin-slow 25s linear infinite',
                'spin-reverse': 'spin-reverse 35s linear infinite',
                'wave': 'wave 2.5s ease-in-out infinite',
                'morph': 'morph 10s ease-in-out infinite',
                'tilt': 'tilt 10s infinite linear',
            },
            keyframes: {
                tilt: {
                    '0%, 50%, 100%': { transform: 'rotate(0deg)' },
                    '25%': { transform: 'rotate(0.5deg)' },
                    '75%': { transform: 'rotate(-0.5deg)' },
                },
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
                    '0%': { opacity: '0', transform: 'scale(0.95)' },
                    '100%': { opacity: '1', transform: 'scale(1)' },
                },
                slideUp: {
                    '0%': { transform: 'translateY(30px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                slideDown: {
                    '0%': { transform: 'translateY(-20px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                'slide-in-right': {
                    '0%': { transform: 'translateX(50px)', opacity: '0' },
                    '100%': { transform: 'translateX(0)', opacity: '1' },
                },
                float: {
                    '0%, 100%': { transform: 'translateY(0) rotate(0deg)' },
                    '25%': { transform: 'translateY(-15px) rotate(1deg)' },
                    '75%': { transform: 'translateY(8px) rotate(-1deg)' },
                },
                'float-slow': {
                    '0%, 100%': { transform: 'translateY(0) scale(1) rotate(0)' },
                    '50%': { transform: 'translateY(-30px) scale(1.05) rotate(2deg)' },
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
                    '0%': { transform: 'scale(0.3)', opacity: '0' },
                    '70%': { transform: 'scale(1.05)' },
                    '100%': { transform: 'scale(1)', opacity: '1' },
                },
                'glow-pulse': {
                    '0%, 100%': { boxShadow: '0 0 20px hsla(var(--glow), 0.3), 0 0 40px hsla(var(--glow), 0.1)' },
                    '50%': { boxShadow: '0 0 40px hsla(var(--glow), 0.5), 0 0 80px hsla(var(--glow), 0.2)' },
                },
                morph: {
                    '0%, 100%': { borderRadius: '60% 40% 30% 70% / 60% 30% 70% 40%' },
                    '50%': { borderRadius: '30% 60% 70% 40% / 50% 60% 30% 60%' },
                },
                'spin-slow': {
                    '0%': { transform: 'rotate(0deg)' },
                    '100%': { transform: 'rotate(360deg)' },
                },
                marquee: {
                    '0%': { transform: 'translateX(0)' },
                    '100%': { transform: 'translateX(-50%)' },
                },
                tilt: {
                    '0%, 50%, 100%': { transform: 'rotate(0deg) translateY(0)' },
                    '25%': { transform: 'rotate(1.5deg) translateY(-10px)' },
                    '75%': { transform: 'rotate(-1.5deg) translateY(10px)' },
                },
            },
            backgroundImage: {
                'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
                'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
                'glass-gradient': 'linear-gradient(135deg, rgba(255, 255, 255, 0.1), rgba(255, 255, 255, 0.05))',
            },
        },
    },
    plugins: [
        require('@kobalte/tailwindcss'),
        ({ addUtilities }) => {
            const newUtilities = {
                '.fill-mode-forwards': {
                    'animation-fill-mode': 'forwards',
                },
                '.fill-mode-backwards': {
                    'animation-fill-mode': 'backwards',
                },
                '.fill-mode-both': {
                    'animation-fill-mode': 'both',
                },
            }
            addUtilities(newUtilities)
        }
    ],
}
