<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import { useFilesStore } from '../stores/useFilesStore'
import { exportToXLSX } from '../utils/exportToXLSX'
import { importFrontmatterFromXlsx, readAndParseMultipleFrontmatter } from '../utils/tauri'

const filesStore = useFilesStore()

async function loadFilesFrontmatter() {
  const temps = await readAndParseMultipleFrontmatter(filesStore.getFileAbsolutePathList())
  Object.keys(temps).forEach((key) => {
    if (!filesStore.files[key]) {
      filesStore.files[key] = {
        relativePath: key,
        frontmatter: {},
        modified: false,
      }
    }
    filesStore.files[key].frontmatter = temps[key]
  })

  filesStore.ready.fileContent = true
}

if (!filesStore.ready.fileContent) {
  await loadFilesFrontmatter()
}

async function importFromXLSX() {
  const selected = await open({
    title: '请选择要导入的 XLSX 文件',
    filters: [
      {
        name: 'Excel 文件',
        extensions: ['xlsx'],
      },
    ],
  })

  if (selected) {
    await importFrontmatterFromXlsx(selected, filesStore.basePath)

    // 重新加载文件内容
    filesStore.ready.fileContent = false

    await loadFilesFrontmatter()
    // 不需要调整 modified 状态，因为导入的内容是直接从 FS 操作的
    filesStore.ready.fileContent = true
  }
}
</script>

<template>
  <div>
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
      <h1>
        导出与导入
      </h1>
      <button class="mt-4 rounded-xl bg-dark px-4 py-2 color-gray-100" @click="() => exportToXLSX(useFilesStore())">
        导出为 XLSX
      </button>
      <button class="mt-4 rounded-xl bg-dark px-4 py-2 color-gray-100" @click="importFromXLSX">
        从 XLSX 导入
      </button>
    </div>
  </div>
</template>
