/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",

  content: [
    "index.html",
    "./_includes/**/*.{html,js}",
    "./_layouts/**/*.{html,js}",
  ],
  theme: {
    extend: {},
  },

  plugins: [require("@tailwindcss/typography")],
};
