/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],

  theme: {
    extend: {
      backgroundColor: {
        black: "#1B1B1B",
        extra_black: "#111111"
      },
      fontFamily: {
        outfit: ["Outfit", "sans-serif"],
      },
      colors: {
        white: "#BABABA",
        text_black: "#222222",
      },
    },
  },
  plugins: [],
};
