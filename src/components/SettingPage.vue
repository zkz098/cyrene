<script setup lang="ts">
import { useLanguage } from '../composables/useLanguage'
import { useFilesStore } from '../stores/useFilesStore'

const { t, currentLanguage, availableLanguages, changeLanguage } = useLanguage()
const filesStore = useFilesStore()

function resetFolder() {
  filesStore.ready.fileTree = false
  filesStore.filetree = []
  filesStore.files = {}
  filesStore.currentAccessPath = ''
  filesStore.basePath = ''
  filesStore.ready.fileList = false
  filesStore.ready.fileTree = false
  filesStore.ready.fileContent = false
  filesStore.ready.selectedFile = false
  filesStore.ready.xlsxExporting = false
}
</script>

<template>
  <div class="flex flex-col items-start justify-start gap-6 p-4">
    <h1 class="text-2xl">
      {{ t('settings.title') }}
    </h1>

    <!-- 语言设置 -->
    <div class="max-w-md w-full">
      <h2 class="mb-3 text-lg">
        {{ t('settings.language') }}
      </h2>
      <div class="flex gap-2">
        <button
          v-for="lang in availableLanguages"
          :key="lang.code"
          class="border rounded px-4 py-2 transition-colors"
          :class="[
            currentLanguage === lang.code
              ? 'border-blue-500 bg-blue-500 text-white'
              : 'border-gray-300 bg-white text-gray-700 hover:bg-gray-50',
          ]"
          @click="changeLanguage(lang.code)"
        >
          {{ lang.name }}
        </button>
      </div>
    </div>

    <!-- 重置按钮 -->
    <div class="max-w-md w-full">
      <h2 class="mb-3 text-lg">
        {{ t('files.title') }}
      </h2>
      <button
        class="rounded bg-gray-700 px-4 py-2 text-white transition-colors hover:bg-gray-800"
        @click="resetFolder"
      >
        {{ t('settings.reset') }}
      </button>
    </div>
  </div>
</template>
