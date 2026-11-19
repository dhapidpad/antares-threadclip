/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        brand: {
          50: '#f0fdf4',
          500: '#22c55e',
          600: '#16a34a',
          700: '#15803d',
        },
        studio: {
          bg: '#0f172a',
          panel: '#1e293b',
          border: '#334155',
          active: '#3b82f6',
        }
      }
    },
  },
  plugins: [],
}
