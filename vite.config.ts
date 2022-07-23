import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import FullReload from 'vite-plugin-full-reload'

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [svelte()],
	build: {
		outDir: './public',
	},
	clearScreen: false,
})
