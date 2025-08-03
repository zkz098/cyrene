import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentLocale, setLocale } from '../i18n'

export function useLanguage() {
  const { t, locale } = useI18n()

  const currentLanguage = computed(() => getCurrentLocale())

  const availableLanguages = [
    { code: 'en-US', name: 'English' },
    { code: 'zh-CN', name: '中文' },
  ]

  function changeLanguage(language: string) {
    setLocale(language)
  }

  function getLanguageName(code: string): string {
    const lang = availableLanguages.find(l => l.code === code)
    return lang ? lang.name : code
  }

  return {
    t,
    locale,
    currentLanguage,
    availableLanguages,
    changeLanguage,
    getLanguageName,
  }
}
