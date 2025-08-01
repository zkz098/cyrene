<script setup lang="ts">
import { ask, message } from '@tauri-apps/plugin-dialog'
import { ref } from 'vue'
import { useFilesStore } from '../stores/useFilesStore'
import { exportToXLSX } from '../utils/exportToXLSX'
import { readAndParseMultipleFrontmatter } from '../utils/tauri'

const filesStore = useFilesStore()

if (!filesStore.ready.fileContent) {
  const temps = await readAndParseMultipleFrontmatter(filesStore.getFileAbsolutePathList())
  Object.keys(temps).forEach((key) => {
    filesStore.files[key].frontmatter = temps[key]
  })

  filesStore.ready.fileContent = true
}

const operation = ref('add') // 默认操作为添加
const fileRegExp = ref(/\.md$/) // 默认正则表达式匹配 Markdown 文件
const operateKey = ref('') // 操作的字段名
const operateValue = ref('') // 操作的字段值

async function operateBatch() {
  if (!operateKey.value || !fileRegExp.value || (operateKey.value === 'normalize' && !operateValue.value)) {
    await message('请填写需要操作的字段名、文件正则和目标值', { title: '错误', kind: 'error' })
    return
  }

  const isBackup = await ask(`你正在进行批量编辑操作，操作错误可能导致严重的后果，尽管你进行的操作不会立刻写入到文件系统中，但建议备份当前的 frontmatter，以防止错误编辑后难以恢复`, { title: '备份确认', kind: 'warning' })
  if (isBackup) {
    await exportToXLSX(useFilesStore())
  }

  const modifiedCnt = ref(0)

  if (operation.value === 'add') {
    modifiedCnt.value = filesStore.addKeyValueToFrontmatter(operateKey.value.split(',').map(str => str.trim()), operateValue.value, fileRegExp.value)
  }
  else if (operation.value === 'remove') {
    modifiedCnt.value = filesStore.removeKeyFromFrontmatter(operateKey.value.split(',').map(str => str.trim()), fileRegExp.value)
  }
  else if (operation.value === 'normalize') {
    modifiedCnt.value = filesStore.normalizeFrontmatter(operateKey.value.split(',').map(str => str.trim()), operateValue.value, fileRegExp.value)
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
      <h1 class="mb-6 text-2xl">
        批量编辑工作台
      </h1>
      <div class="flex flex-col items-start justify-start">
        <span>要执行的操作：</span>
        <label>
          <input v-model="operation" type="radio" value="add">
          添加字段
        </label>
        <label>
          <input v-model="operation" type="radio" value="remove">
          删除字段
        </label>
        <label>
          <input v-model="operation" type="radio" value="normalize">
          归一化字段
        </label>
      </div>
      <label>
        需要进行操作的文件的正则表达式匹配（默认为 Markdown 文件）：<br>
        <input v-model="fileRegExp" class="rounded" type="text">
      </label>
      <div class="flex flex-col items-start justify-start">
        <span>操作内容：</span>
        <label>
          添加、删除或归一化的字段名，存在多个使用逗号分隔 (如：title, author)：<br>
          <input v-model="operateKey" class="rounded" type="text">
        </label>
        <label>
          <span v-if="operation === 'add'">添加的字段值</span>
          <span v-else-if="operation === 'normalize'">归一化的目标值</span>
          <br>
          <input v-if="operation === 'add' || operation === 'normalize'" v-model="operateValue" type="text">
        </label>
      </div>
      <div>
        <button class="mt-4 rounded-xl bg-dark px-4 py-2 color-gray-100" @click="operateBatch">
          执行批量编辑
        </button>
      </div>
    </div>
  </div>
</template>
