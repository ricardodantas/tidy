# Tidy Website

The official website for Tidy, built with [Astro](https://astro.build).

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## 📁 Project Structure

```
website/
├── public/
│   ├── demo.gif            # Main demo animation
│   ├── demo-themes.gif     # Theme switching demo
│   ├── screenshots/        # TUI screenshots
│   └── favicon.svg
├── src/
│   ├── layouts/
│   │   └── Layout.astro    # Base layout with nav and footer
│   └── pages/
│       ├── index.astro     # Home page
│       ├── docs.astro      # Documentation
│       └── themes.astro    # Theme gallery
├── astro.config.mjs        # Astro configuration
└── package.json
```

## 🌐 Deploying to GitHub Pages

The website is automatically deployed via GitHub Actions when you push changes to the `main` branch.

### First-time Setup

1. Go to your repository Settings → Pages
2. Under "Build and deployment", select:
   - **Source**: GitHub Actions
3. Push to `main` to trigger a deployment

### Manual Deployment

If you need to deploy manually:

```bash
# Build the site
npm run build

# The output will be in dist/
# Upload the contents of dist/ to your hosting provider
```

### Configuration

The site is configured to work with the `/tidy` base path for GitHub Pages. If you're hosting elsewhere, update `astro.config.mjs`:

```js
export default defineConfig({
  site: 'https://your-domain.com',
  base: '/', // or your base path
});
```

## 🎨 Design

- **Theme**: Dark mode by default using Dracula-inspired colors
- **Fonts**: Inter (sans-serif) + JetBrains Mono (monospace)
- **Style**: Clean, modern, minimal with smooth animations

### Colors (Dracula palette)

```css
--bg-primary: #282a36;
--bg-secondary: #1e1f29;
--bg-tertiary: #44475a;
--text-primary: #f8f8f2;
--text-secondary: #6272a4;
--accent-purple: #bd93f9;
--accent-pink: #ff79c6;
--accent-cyan: #8be9fd;
--accent-green: #50fa7b;
--accent-yellow: #f1fa8c;
--accent-orange: #ffb86c;
--accent-red: #ff5555;
```

## ✨ Features

- **Copy to clipboard**: Install commands have click-to-copy functionality
- **Smooth scrolling**: Anchor links scroll smoothly to sections
- **Responsive**: Works on all screen sizes
- **Fast**: Static site with minimal JavaScript
- **Accessible**: Semantic HTML with ARIA labels

## 📝 Updating Content

### Adding screenshots

1. Add new screenshots to the root `screenshots/` directory
2. They'll be automatically copied during the build process
3. Reference them in your pages: `{baseUrl}screenshots/filename.png`

### Adding themes to the gallery

Edit `src/pages/themes.astro` and add a new entry to the `themes` array:

```js
{
  name: 'Theme Name',
  slug: 'theme-slug',
  description: 'Short description',
  colors: ['#bg', '#color1', '#color2', '#color3', '#color4']
}
```

## 📄 License

MIT - Same as the main Tidy project.
