<script setup lang="ts">
import { message, open, save } from '@tauri-apps/plugin-dialog'
import { useLanguage } from '../composables/useLanguage'
import { useFilesStore } from '../stores/useFilesStore'
import { exportToXLSX } from '../utils/exportToXLSX'
import { backupFilesAsTarZst, importFrontmatterFromXlsx, readAndParseMultipleFrontmatter } from '../utils/tauri'
import Button from './basic/Button.vue'
import Divider from './basic/Divider.vue'

const { t } = useLanguage()
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
    title: t('export.importExport.selectXlsxFile'),
    filters: [
      {
        name: t('export.importExport.excelFiles'),
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

async function backupFiles() {
  const selected = await save({
    title: '请选择备份文件的保存位置',
    filters: [
      {
        name: 'Zstd 压缩文件',
        extensions: ['zst'],
      },
    ],
    defaultPath: `backup-${new Date().toISOString().split('T')[0]}.tar.zst`,
  })

  if (selected) {
    await backupFilesAsTarZst(filesStore.getFileRelativePathList(), filesStore.basePath, selected, 3)
    await message(t('export.importExport.backupSuccess', { file: selected }))
  }
}
</script>

<template>
  <div>
    <div v-if="!filesStore.ready.fileContent">
      <h1 class="text-2xl">
        {{ t('export.importExport.loadingContent') }}
      </h1>
      <p>
        {{ t('export.importExport.loadingDescription') }}
      </p>
    </div>
    <div v-else class="flex flex-col items-start justify-start p-4">
      <h1>
        {{ t('export.importExport.title') }}
      </h1>
      <Button class="mt-4 bg-green-6 hover:bg-green-7" @click="() => exportToXLSX(useFilesStore())">
        {{ t('export.importExport.exportToXlsx') }}
      </Button>
      <Button class="mt-4 bg-green-6 hover:bg-green-7" @click="importFromXLSX">
        {{ t('export.importExport.importFromXlsx') }}
      </Button>

      <Divider />

      <Button class="mt-4 bg-amber-4 hover:bg-amber-5" @click="backupFiles">
        {{ t('export.importExport.backup') }}
      </Button>
    </div>
  </div>
</template>
