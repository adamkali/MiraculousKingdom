/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
        fontFamily: 
        { "victo-mono": ['Victor Mono']},
        screens: {
        '3xl': '1910px',
        '4xl': '2560px',
      },
    }
  },
  plugins: [
    // ...
    require('tailwind-scrollbar'),
  ]
}
