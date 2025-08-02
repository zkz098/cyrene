<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useFilesStore } from '../stores/useFilesStore'
import { buildFileTree } from '../utils/buildFileTree'

import { getRelativePath } from '../utils/getRelativePath'
import { readAndParseYamlFrontmatter } from '../utils/tauri'
import JsonEditor from './tree/jsonEditor.vue'
import TreeNode from './tree/TreeNode.vue'

const { t } = useI18n()
const filesStore = useFilesStore()

if (!filesStore.ready.fileTree) {
  filesStore.filetree = buildFileTree(filesStore.getFileRelativePathList())
  filesStore.ready.fileTree = true
}

const jsonContent = ref('')

watch(() => filesStore.currentAccessPath, async (newPath) => {
  if (newPath) {
    if (filesStore.files[newPath]?.frontmatter) {
      jsonContent.value = JSON.stringify(filesStore.files[newPath].frontmatter, null, 2)
    }
    else {
      if (!filesStore.files[newPath]) {
        filesStore.files[newPath] = { frontmatter: {}, relativePath: getRelativePath(newPath, filesStore.basePath), modified: false }
      }
      filesStore.files[newPath].frontmatter = await readAndParseYamlFrontmatter(newPath)
      jsonContent.value = JSON.stringify(filesStore.files[newPath].frontmatter, null, 2)
    }
  }
  else {
    // 设置默认的提示内容
    jsonContent.value = JSON.stringify({
      name: t('files.defaultName'),
      description: t('files.defaultDescription'),
    }, null, 2)
  }
}, { immediate: true })
</script>

<template>
  <div v-if="!filesStore.ready.fileTree" class="flex flex-col items-center justify-center p-4">
    <h1 class="text-2xl">
      {{ t('files.loading') }}
    </h1>
    <p class="text-gray-600">
      {{ t('files.loadingDescription') }}
    </p>
  </div>
  <div v-else class="flex flex-col items-start justify-start p-4">
    <div class="mb-4 w-full flex flex-row items-center justify-between">
      <ul class="h-90vh min-w-30vw flex flex-col list-none items-start justify-start gap-2 overflow-scroll">
        <TreeNode v-for="(node, index) in filesStore.filetree" :key="index" :node="node" />
      </ul>
      <div class="h-100% w-full flex flex-col items-start justify-start">
        <JsonEditor
          v-model="jsonContent"
          :readonly="false"
          theme="github-light"
          :placeholder="t('files.jsonPlaceholder')"
        />
        <button class="m-2 mt-2 rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600" @click="filesStore.files[filesStore.currentAccessPath].frontmatter = JSON.parse(jsonContent)">
          {{ t('files.saveToMemory') }}
        </button>
      </div>
    </div>
  </div>
</template>
