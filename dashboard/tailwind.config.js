/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#0a0e27',
        surface: '#111827',
        border: '#1e293b',
        primary: '#ffffff',
        secondary: '#b0b8d4',
        accent: {
          teal: '#00d9ff',
          indigo: '#4f46e5',
          secondary: '#6366f1'
        }
      },
      boxShadow: {
        'glow-teal': '0 0 15px rgba(0, 217, 255, 0.5)',
        'glow-indigo': '0 0 15px rgba(79, 70, 229, 0.5)',
      }
    },
  },
  plugins: [],
}
