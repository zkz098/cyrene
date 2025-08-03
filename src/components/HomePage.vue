<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useFilesStore } from '../stores/useFilesStore'
import { getRelativePath } from '../utils/getRelativePath'
import { getAllFilesOfDir } from '../utils/tauri'

const { t } = useI18n()
const folderPath = ref('')
const filesStore = useFilesStore()

async function selectFolder() {
  const selected = await open({
    directory: true,
    title: t('files.selectFolder'),
  })

  if (selected) {
    folderPath.value = selected
    filesStore.ready.selectedFile = true
    filesStore.basePath = selected // 设置基准路径
    filesStore.ready.fileList = true
  }

  (await getAllFilesOfDir(folderPath.value)).forEach((temp) => {
    temp = temp.replace(/\\/g, '/') // 替换反斜杠为正斜杠
    filesStore.files[temp] = {
      relativePath: getRelativePath(temp, folderPath.value),
      frontmatter: {},
      modified: false,
    }
  })
}
</script>

<template>
  <div>
    <div v-if="!filesStore.ready.selectedFile && !filesStore.ready.fileList" class="flex flex-col items-start justify-start p-4">
      <h1 class="mb-4 text-3xl font-bold">
        {{ t('home.welcomeMessage') }}
      </h1>
      <p class="mb-4 text-gray-600">
        {{ t('home.description') }}
      </p>

      <button
        class="mt-4 rounded-xl bg-gray-700 px-4 py-2 text-white transition-colors hover:bg-gray-800"
        @click="selectFolder"
      >
        {{ t('home.getStarted') }}
      </button>
    </div>

    <div v-else-if="filesStore.ready.fileList" class="flex flex-col items-center justify-center p-4">
      <h1 class="mb-4 text-2xl">
        {{ t('files.openFolder') }}: {{ filesStore.basePath }}
      </h1>
      <div class="max-w-2xl text-left">
        <p class="mb-4 leading-relaxed">
          {{ t('home.instructionsStart') }}<br>
          {{ t('home.fileTreeInstruction') }}<br>
          {{ t('home.exportInstruction') }}<br>
          {{ t('home.editInstruction') }}<br>
          {{ t('home.settingsInstruction') }}<br>
          {{ t('home.saveInstruction') }}
        </p>
      </div>
    </div>

    <div v-else-if="filesStore.ready.selectedFile && !filesStore.ready.fileList" class="flex flex-col items-center justify-center p-4">
      <h1 class="mb-4 text-2xl">
        {{ t('common.loading') }}
      </h1>
      <p class="text-gray-600">
        {{ t('home.loadingMessage') }}
      </p>
    </div>
  </div>
</template>
