import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [svelte()],
	// root: "./public",
	// base: "./public",
	build: {
		outDir: './public',
	},
})
