/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./app/src/**/*.rs"],
    transform: {
      'rs': (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
    purge: {
      enabled: true,
      content: ["./app/src/**/*.rs", "./app/src/**/*.html"],
    }
  },
  plugins: [],
  theme: {
    colors: {
    },
    fontFamily:{
      sans: ['Minecraft', 'sans-serif'],
    }
  },
  extend:{
    colors: {
      tpop: {
        blue: '#3f6d9e',
        purple: '#151640',
        pink: '#f783b0',
        white: '#e6f2ef',
      },
      cmyk: {
        cyan: '#35cbc8',
        magenta: '#c93864',
        yellow: '#ffdb85',
        black: '#1b192a'
      },
      baldur: {
        black: '#0f1b26',
        white: '#f5e8d1',
        blue: '#20a5a6',
        orange: '#dd5639',
      },
      kaneki: {
        white: '#ffffff',
        orange: '#f42e1f',
        purple: '#2f256b',
        black: '#060608'
      },
      pacific: {
        purple: '#384e96',
        blue: '#2c99cc',
        cyan: '#6cdcd5',
        white: '#f4f3de',
      }
    }
  }
}
