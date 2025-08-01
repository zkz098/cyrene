<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import { ref } from 'vue'
import { useFilesStore } from '../stores/useFilesStore'
import { getRelativePath } from '../utils/getRelativePath'
import { getAllFilesOfDir } from '../utils/tauri'

const folderPath = ref('')
const filesStore = useFilesStore()

async function selectFolder() {
  const selected = await open({
    directory: true,
    title: '请选择一个文件夹',
  })

  if (selected) {
    folderPath.value = selected
    filesStore.ready.selectedFile = true
    filesStore.basePath = selected // 设置基准路径
    filesStore.ready.fileList = true
  }

  (await getAllFilesOfDir(folderPath.value)).forEach((temp) => {
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
      <h1>欢迎使用 Frontmatter-Editor</h1>

      <button class="mt-4 rounded-xl bg-dark px-4 py-2 color-gray-100" @click="selectFolder">
        选择你存储文章的文件夹以开始
      </button>
    </div>
    <div v-else-if="filesStore.ready.fileList" class="flex flex-col items-center justify-center p-4">
      <h1 class="text-2xl">
        已选择文件夹: {{ filesStore.basePath }}
      </h1>
      <p class="mt-2">
        现在你可以开始编辑了：<br>
        左侧的树形图标是文件列表，点击可以查看和编辑文件内容。<br>
        导出按钮可以将所有文件导出为 XLSX 格式或从 XLSX 导入所有文件的 frontmatter。<br>
        编辑按钮可以批量编辑 frontmatter 字段。<br>
        设置按钮可以配置一些选项，比如语言等。<br>
        保存按钮可以将所有修改过的文件保存到文件系统中。
      </p>
    </div>
    <div v-else-if="filesStore.ready.selectedFile && !filesStore.ready.fileList" class="flex flex-col items-center justify-center p-4">
      <h1 class="text-2xl">
        正在加载...
      </h1>
      <p>请稍等片刻，文件列表正在准备中。</p>
    </div>
  </div>
</template>
