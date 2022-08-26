import { defineConfig } from 'windicss/helpers'
import formsPlugin from 'windicss/plugin/forms'


export default defineConfig({
    safelist: 'p-3 p-4 p-5',
    theme: {
      extend: {
        colors: {
          'dark': '#2A3740',
          'white': '#ECEFF8',
          'black': '#1E212A',
          'green': '#25dc63',
        },
        fontFamily: {
          'mclaren': ['McLaren', 'cursive'],
        }
      }
    },
    plugins: [formsPlugin],
  })