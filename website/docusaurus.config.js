// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const {themes} = require('prism-react-renderer');
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'PwGen-rust v1.2',
  tagline: 'Advanced Password & Secrets Manager built in Rust - Now 30-40% smaller!',
  favicon: 'img/favicon.png',

  // Set the production url of your site here
  url: 'https://pwgenrust.dev',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'hxhippy', // Usually your GitHub org/user name.
  projectName: 'pwgen', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/hxhippy/pwgen/tree/main/website/',
        },
        blog: {
          showReadingTime: true,
          feedOptions: {
            type: 'all',
            copyright: `Copyright © ${new Date().getFullYear()} HxHippy, Kief Studio, TRaViS.`,
          },
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/hxhippy/pwgen/tree/main/website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/pwgen-social-card.jpg',
      navbar: {
        title: 'PwGen-rust',
        logo: {
          alt: 'PwGen Logo',
          src: 'img/PWGenLogo.png',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Documentation',
          },
          {
            to: '/download',
            label: 'Download',
            position: 'left'
          },
          {
            to: '/blog', 
            label: 'Blog', 
            position: 'left'
          },
          {
            href: 'https://github.com/hxhippy/pwgen',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/getting-started/installation',
              },
              {
                label: 'User Guide',
                to: '/docs/user-guide/passwords',
              },
              {
                label: 'CLI Reference',
                to: '/docs/cli/overview',
              },
              {
                label: 'Security',
                to: '/docs/security/architecture',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'GitHub Discussions',
                href: 'https://github.com/hxhippy/pwgen/discussions',
              },
              {
                label: 'Issues',
                href: 'https://github.com/hxhippy/pwgen/issues',
              },
              {
                label: 'Contributing',
                href: 'https://github.com/hxhippy/pwgen/blob/main/CONTRIBUTING.md',
              },
            ],
          },
          {
            title: 'Powered By',
            items: [
              {
                label: 'TRaViS - AI-Powered EASM',
                href: 'https://travisasm.com',
              },
              {
                label: 'Kief Studio - AI Integration',
                href: 'https://kief.studio',
              },
              {
                label: 'HxHippy',
                href: 'https://hxhippy.com',
              },
              {
                label: '@HxHippy on X',
                href: 'https://x.com/HxHippy',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                to: '/blog',
              },
              {
                label: 'Security Policy',
                href: 'https://github.com/hxhippy/pwgen/blob/main/SECURITY.md',
              },
              {
                label: 'License',
                href: 'https://github.com/hxhippy/pwgen/blob/main/LICENSE',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} HxHippy, Kief Studio, TRaViS. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'toml', 'bash'],
      },
      algolia: {
        // The application ID provided by Algolia
        appId: 'YOUR_APP_ID',
        // Public API key: it is safe to commit it
        apiKey: 'YOUR_SEARCH_API_KEY',
        indexName: 'pwgenrust',
        // Optional: see doc section below
        contextualSearch: true,
        // Optional: Specify domains where the navigation should occur through window.location instead on history.push. Useful when our Algolia config crawls multiple documentation sites and we want to navigate with window.location.href to them.
        externalUrlRegex: 'external\\.com|domain\\.com',
        // Optional: Algolia search parameters
        searchParameters: {},
        // Optional: path for search page that enabled by default (`false` to disable it)
        searchPagePath: 'search',
      },
      announcementBar: {
        id: 'v1_2_release',
        content:
          '🎉 PwGen v1.2 is out! 30-40% smaller binaries, enhanced security, and flexible builds! <a target="_blank" rel="noopener noreferrer" href="https://github.com/hxhippy/pwgen">Download now</a> ⭐️',
        backgroundColor: '#25c2a0',
        textColor: '#FFFFFF',
        isCloseable: true,
      },
    }),
};

module.exports = config;