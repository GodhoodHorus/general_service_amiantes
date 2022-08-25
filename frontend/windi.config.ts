import { defineConfig } from 'windicss/helpers'
import formsPlugin from 'windicss/plugin/forms'


export default defineConfig({
    safelist: 'p-3 p-4 p-5',
    theme: {},
    plugins: [formsPlugin],
  })