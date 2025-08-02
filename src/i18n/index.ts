import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import zh from './locales/zh.json'

const messages = {
  en,
  zh,
}

// 获取浏览器语言或从localStorage读取用户设置
function getDefaultLocale(): string {
  const savedLocale = localStorage.getItem('locale')
  if (savedLocale && ['en', 'zh'].includes(savedLocale)) {
    return savedLocale
  }

  const browserLocale = navigator.language.toLowerCase()
  if (browserLocale.startsWith('zh')) {
    return 'zh'
  }

  return 'en'
}

const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: 'en',
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
  return 'en'
}
