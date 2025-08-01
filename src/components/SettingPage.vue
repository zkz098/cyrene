<script setup lang="ts">
import { useFilesStore } from '../stores/useFilesStore'

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
  <div class="flex flex-col items-start justify-start p-4">
    <h1 class="text-2xl">
      设置
    </h1>
    <button class="m-2 mt-2 rounded bg-gray-700 px-4 py-2 text-white hover:bg-gray-800" @click="resetFolder">
      重置文章目录
    </button>
  </div>
</template>
