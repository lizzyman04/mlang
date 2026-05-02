import { defineConfig } from 'vitepress'

export default defineConfig({
  base: '/mlang/',
  title: 'MLang',
  description: 'A math-first, statically typed scripting language built in Rust.',
  head: [
    ['link', { rel: 'icon', href: '/mlang/favicon.ico' }],
    ['meta', { name: 'theme-color', content: '#646cff' }],
  ],

  themeConfig: {
    logo: '/logo.svg',
    siteTitle: 'MLang',

    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Reference', link: '/reference/syntax' },
      {
        text: 'GitHub',
        link: 'https://github.com/lizzyman04/mlang',
      },
    ],

    sidebar: [
      {
        text: 'Getting Started',
        items: [
          { text: 'Introduction', link: '/guide/getting-started' },
          { text: 'Installation', link: '/guide/installation' },
          { text: 'Examples', link: '/guide/examples' },
        ],
      },
      {
        text: 'Language Reference',
        items: [
          { text: 'Syntax Overview', link: '/reference/syntax' },
          { text: 'Types', link: '/reference/types' },
          { text: 'Functions', link: '/reference/functions' },
          { text: 'Structs', link: '/reference/structs' },
          { text: 'I/O', link: '/reference/io' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/lizzyman04/mlang' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2025 Lizzyman04',
    },

    editLink: {
      pattern: 'https://github.com/lizzyman04/mlang/edit/main/landing/:path',
      text: 'Edit this page on GitHub',
    },

    search: {
      provider: 'local',
    },
  },
})
