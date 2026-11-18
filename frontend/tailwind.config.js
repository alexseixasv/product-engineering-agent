/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        ink: "#14213d",
        paper: "#f8f5f0",
        accent: "#c44900",
        line: "#d9d2c5",
      },
      fontFamily: {
        display: ["\"Source Serif 4\"", "Georgia", "serif"],
        body: ["\"IBM Plex Sans\"", "Segoe UI", "sans-serif"],
      },
    },
  },
  plugins: [],
};
