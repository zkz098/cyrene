<script setup lang="ts">
import { useFilesStore } from '../stores/useFilesStore'
import { readAndParseMultipleFrontmatter } from '../utils/tauri'

const filesStore = useFilesStore()

if (!filesStore.ready.fileContent) {
  const temps = await readAndParseMultipleFrontmatter(filesStore.getFileAbsolutePathList())
  Object.keys(temps).forEach((key) => {
    filesStore.files[key].frontmatter = temps[key]
  })

  filesStore.ready.fileContent = true
}
</script>

<template>
  <div v-if="!filesStore.ready.fileContent">
    <h1 class="text-2xl">
      正在读取并解析所有文件的 frontmatter
    </h1>
    <p>
      此过程是并行进行的，通过不会花费太长时间。<br>
      如果你有很多文件，可能需要几分钟时间。<br>
    </p>
  </div>
  <div v-else class="flex flex-col items-start justify-start p-4">
    <h1 class="mb-6 text-2xl">
      批量编辑工作台
    </h1>
  </div>
</template>
