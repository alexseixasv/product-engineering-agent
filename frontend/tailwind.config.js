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
        deal: {
          primary: "#30a83c",
          "green-strong": "#1f7a2d",
          hot: "#e53935",
          cold: "#1e88e5",
          warm: "#fb8c00",
          premium: "#7e57c2",
          discount: "#e6f6e9",
          canvas: "#eef1f4",
          surface: "#ffffff",
          hairline: "#d9dee5",
          ink: "#0b0b0b",
          body: "#5c6670",
          muted: "#9aa3ad",
          "warning-surface": "#fff4d6",
          "warning-text": "#8a5a00",
        },
      },
      fontFamily: {
        display: ["\"Source Serif 4\"", "Georgia", "serif"],
        body: ["\"IBM Plex Sans\"", "Segoe UI", "sans-serif"],
        deal: ["Mulish", "Nunito Sans", "Segoe UI", "sans-serif"],
      },
    },
  },
  plugins: [],
};
