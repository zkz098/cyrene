import { createI18n } from 'vue-i18n'
import enUs from './locales/en-US.json'
import zhCn from './locales/zh-CN.json'

const messages = {
  'en-US': enUs,
  'zh-CN': zhCn,
}

// 获取浏览器语言或从localStorage读取用户设置
function getDefaultLocale(): string {
  const savedLocale = localStorage.getItem('locale')
  if (savedLocale && ['en-US', 'zh-CN'].includes(savedLocale)) {
    return savedLocale
  }

  const browserLocale = navigator.language.toLowerCase()
  if (browserLocale.startsWith('zh')) {
    return 'zh-CN'
  }

  return 'en-US'
}

const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: 'en-US',
  messages,
})

export default i18n

// 切换语言的辅助函数
export function setLocale(locale: string) {
  if (i18n.global.locale && typeof i18n.global.locale === 'object') {
    i18n.global.locale.value = locale as any
  }
  localStorage.setItem('locale', locale)
}

export function getCurrentLocale(): string {
  if (i18n.global.locale && typeof i18n.global.locale === 'object') {
    return i18n.global.locale.value as string
  }
  return 'en-US'
}
