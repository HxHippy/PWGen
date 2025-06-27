/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  tutorialSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/installation',
        'getting-started/first-run',
        'getting-started/basic-usage',
      ],
    },
    {
      type: 'category',
      label: 'User Guide',
      items: [
        'user-guide/passwords',
        'user-guide/secrets',
        'user-guide/generator',
        'user-guide/import-export',
        'user-guide/backup-restore',
        'user-guide/configuration',
        'user-guide/troubleshooting',
      ],
    },
    {
      type: 'category',
      label: 'CLI Reference',
      items: [
        'cli/overview',
        'cli/commands',
        'cli/configuration',
      ],
    },
    {
      type: 'category',
      label: 'Security',
      items: [
        'security/architecture',
        'security/encryption',
        'security/best-practices',
        'security/reporting',
      ],
    },
    {
      type: 'category',
      label: 'Developers',
      items: [
        'developers/building',
        'developers/architecture',
        'developers/api',
        'developers/contributing',
      ],
    },
    {
      type: 'category',
      label: 'Tutorials',
      items: [
        'tutorials/migration',
        'tutorials/team-usage',
        'tutorials/automation',
      ],
    },
  ],

  // But you can create a sidebar manually
  /*
  tutorialSidebar: [
    'intro',
    'hello',
    {
      type: 'category',
      label: 'Tutorial',
      items: ['tutorial-basics/create-a-document'],
    },
  ],
   */
};

module.exports = sidebars;