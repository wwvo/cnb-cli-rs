import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'CNB CLI',
  description: 'CNB 平台专属命令行工具文档',
  lang: 'zh-CN',
  lastUpdated: true,
  cleanUrls: true,

  themeConfig: {
    nav: [{ text: '指南', link: '/guide/getting-started', activeMatch: '/guide/' }],

    sidebar: [
      {
        text: '入门',
        items: [{ text: '快速开始', link: '/guide/getting-started' }],
      },
    ],

    socialLinks: [{ icon: 'github', link: 'https://cnb.cool/prevailna/cnb' }],

    footer: {
      message: 'CNB 平台专属命令行工具',
    },

    outline: {
      label: '页面导航',
    },

    lastUpdated: {
      text: '最后更新于',
    },

    docFooter: {
      prev: '上一页',
      next: '下一页',
    },

    search: {
      provider: 'local',
      options: {
        translations: {
          button: { buttonText: '搜索文档' },
          modal: {
            noResultsText: '无法找到相关结果',
            resetButtonTitle: '清除查询条件',
            footer: { selectText: '选择', navigateText: '切换' },
          },
        },
      },
    },
  },
})
