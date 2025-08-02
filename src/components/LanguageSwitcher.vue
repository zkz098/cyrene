<script setup lang="ts">
import { useLanguage } from '../composables/useLanguage'

const { currentLanguage, availableLanguages, changeLanguage } = useLanguage()
</script>

<template>
  <div class="relative inline-block">
    <select
      :value="currentLanguage"
      class="appearance-none border border-gray-300 rounded bg-white px-3 py-2 pr-8 text-gray-700 shadow focus:border-blue-500 hover:border-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-200"
      @change="changeLanguage(($event.target as HTMLSelectElement).value)"
    >
      <option
        v-for="lang in availableLanguages"
        :key="lang.code"
        :value="lang.code"
      >
        {{ lang.name }}
      </option>
    </select>
    <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2">
      <div class="i-ri-arrow-down-s-line text-gray-400" />
    </div>
  </div>
</template>
