/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],

  theme: {
    extend: {
      backgroundColor: {
        black: "#1B1B1B",
      },
      fontFamily: {
        outfit: ["Outfit", "sans-serif"],
      },
      colors: {
        "button-black": "#0A0A0A",
        "stroke-black": "#332E2E",
        "button-white": "#D9D9D9",
        "stroke-white": "#C3C3C3",
      },
    },
  },
  plugins: [],
};
