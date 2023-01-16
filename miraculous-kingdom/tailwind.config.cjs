/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme');
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    fontFamily: {
        'sans': ['Victor Mono', ...defaultTheme.fontFamily.sans] 
    },
    extend: {},
  },
  plugins: [],
}
