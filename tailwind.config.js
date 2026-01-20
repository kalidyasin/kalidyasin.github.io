/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        tokyonight: {
          bg: "#1a1b26",
          fg: "#a9b1d6",
          black: "#32344a",
          red: "#f7768e",
          green: "#9ece6a",
          yellow: "#e0af68",
          blue: "#7aa2f7",
          magenta: "#bb9af7",
          cyan: "#7dcfff",
          white: "#787c99",
          bright_black: "#444b6a",
          bright_red: "#ff7a93",
          bright_green: "#b9f27c",
          bright_yellow: "#ff9e64",
          bright_blue: "#7da6ff",
          bright_magenta: "#bb9af7",
          bright_cyan: "#0db9d7",
          bright_white: "#acb0d0",
        },
      },
      fontFamily: {
        mono: ['"JetBrains Mono"', 'monospace'],
      },
    },
  },
  plugins: [],
}
