# PwGen Website

This is the official website for PwGen built with [Docusaurus](https://docusaurus.io/), hosted at [pwgenrust.dev](https://pwgenrust.dev).

## 🚀 Development

### Prerequisites

- Node.js 18 or later
- npm or yarn

### Getting Started

```bash
# Install dependencies
npm install

# Start development server
npm start
```

The website will open at `http://localhost:3000`.

### Building

```bash
# Build for production
npm run build

# Serve locally
npm run serve
```

## 📁 Structure

```
website/
├── blog/                    # Blog posts
├── docs/                    # Documentation
├── src/
│   ├── components/          # React components
│   ├── css/                 # Global styles
│   └── pages/              # Custom pages
├── static/                  # Static assets
├── docusaurus.config.js     # Configuration
└── sidebars.js             # Documentation navigation
```

## ✨ Features

- **Fast & Modern**: Built with React and Docusaurus
- **SEO Optimized**: Meta tags, sitemaps, structured data
- **Mobile Responsive**: Works on all devices
- **Dark Mode**: Automatic theme switching
- **Search**: Full-text search with Algolia
- **Blog**: Release notes and announcements
- **Documentation**: Comprehensive guides and API reference

## 🎨 Customization

### Branding

Brand assets are located in:
- `static/img/` - Logos and icons
- `src/css/custom.css` - Brand colors and styles

### Content

- **Homepage**: `src/pages/index.js`
- **Downloads**: `src/pages/download.js`
- **Documentation**: `docs/` directory
- **Blog**: `blog/` directory

## 🚀 Deployment

### GitHub Pages (Automated)

The website is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.

### Manual Deployment

```bash
# Build the website
npm run build

# Deploy to GitHub Pages
npm run deploy
```

### Custom Domain

To use a custom domain:

1. Add `CNAME` file to `static/` directory
2. Configure DNS records
3. Update `url` in `docusaurus.config.js`

## 📊 Analytics

The website includes:

- **Google Analytics** - Traffic and user behavior
- **Search Analytics** - Search queries and results
- **Performance Monitoring** - Core Web Vitals

## 🔧 Configuration

Key configuration files:

- `docusaurus.config.js` - Main configuration
- `sidebars.js` - Documentation navigation
- `src/css/custom.css` - Custom styles
- `package.json` - Dependencies and scripts

## 🤝 Contributing

See the main [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

### Content Guidelines

- Use clear, concise language
- Include code examples where helpful
- Follow the existing structure and style
- Test all links and code samples

### Writing Documentation

- Place files in appropriate `docs/` subdirectories
- Update `sidebars.js` for navigation
- Use frontmatter for metadata
- Include screenshots for UI features

### Blog Posts

- Create files in `blog/` directory
- Use date prefix: `YYYY-MM-DD-title.md`
- Include author information
- Add relevant tags

## 📄 License

This website content is licensed under [Apache 2.0](../LICENSE).

## 🆘 Support

- 🐛 [Report Issues](https://github.com/your-username/pwgen/issues)
- 💬 [Discussions](https://github.com/your-username/pwgen/discussions)
- 📧 [Contact](mailto:team@pwgenrust.dev)

---

Built with ❤️ and ⚡ by the PwGen team