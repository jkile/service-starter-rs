/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./controllers/templates/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
