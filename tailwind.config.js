/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
    // tal vez otros paths si usas SSR, templates, etc.
  ],
  theme: {
    extend: {
      // tus customizaciones de tema si quieres
    },
  },
  plugins: [
    require("daisyui")
  ],
}
