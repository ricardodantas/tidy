// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  site: 'https://ricardodantas.github.io',
  base: '/tidy',
  build: {
    assets: 'assets'
  }
});
