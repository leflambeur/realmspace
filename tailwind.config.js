/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./app/src/**/*.rs", "./app/src/**/*.css" ],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {
      fontFamily: {
        minecraft: ['Minecraft', 'sans-serif'],
      },
      colors: {
        primary: {
          bg: '#c381b5',
          text: '#fefcd0',
          shadow: '#fefcd0',
        },
        secondary: {
          bg: '#fefcd0',
          text: '#000000',
          shadow: '#c381b5',
        },
        outline: {
          text: '#000000',
        },
      },
    },
  },
  plugins: [],
}
