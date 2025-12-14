import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Strata Engineering Docs',
  url: 'https://kk5123.github.io',
  baseUrl: '/',
  organizationName: 'kk5123',
  projectName: 'strata',
  i18n: { defaultLocale: 'ja', locales: ['ja'] },

  future: { v4: true, },
  onBrokenLinks: 'throw',

  presets: [
    [
      'classic', {
        docs: { path: './pages', sidebarPath: './sidebars.ts', routeBasePath: '/', },
        blog: false,
        theme: { customCss: './assets/css/custom.css' },
      } satisfies Preset.Options,
    ]
  ],

  themeConfig: {
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,

  markdown: {
    mermaid: true,
  },
  themes: ["@docusaurus/theme-mermaid"],    
};

export default config;
