<script setup lang="ts">
import { ask, message } from '@tauri-apps/plugin-dialog'
import { ref } from 'vue'
import { useLanguage } from '../composables/useLanguage'
import { useFilesStore } from '../stores/useFilesStore'
import { exportToXLSX } from '../utils/exportToXLSX'
import { readAndParseMultipleFrontmatter } from '../utils/tauri'

import Button from './basic/Button.vue'

const { t } = useLanguage()
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
    await message(t('edit.batchEdit.fillFieldsError'), { title: t('common.error'), kind: 'error' })
    return
  }

  const isBackup = await ask(t('edit.batchEdit.backupConfirm'), { title: t('edit.batchEdit.backupTitle'), kind: 'warning' })
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

  await message(t('edit.batchEdit.modifiedFilesMessage', { count: modifiedCnt.value }), { title: t('edit.batchEdit.operationComplete'), kind: 'info' })
}
</script>

<template>
  <div>
    <div v-if="!filesStore.ready.fileContent">
      <h1 class="text-2xl">
        {{ t('edit.batchEdit.loadingContent') }}
      </h1>
      <p>
        {{ t('edit.batchEdit.loadingDescription') }}
      </p>
    </div>
    <div v-else class="flex flex-col items-start justify-start p-4">
      <h1 class="mb-6 text-2xl">
        {{ t('edit.batchEdit.title') }}
      </h1>
      <div class="flex flex-col items-start justify-start">
        <span>{{ t('edit.batchEdit.operation') }}</span>
        <label>
          <input v-model="operation" type="radio" value="add">
          {{ t('edit.batchEdit.addField') }}
        </label>
        <label>
          <input v-model="operation" type="radio" value="remove">
          {{ t('edit.batchEdit.removeField') }}
        </label>
        <label>
          <input v-model="operation" type="radio" value="normalize">
          {{ t('edit.batchEdit.normalizeField') }}
        </label>
      </div>
      <label>
        {{ t('edit.batchEdit.fileRegexLabel') }}<br>
        <input v-model="fileRegExp" class="rounded" type="text">
      </label>
      <div class="flex flex-col items-start justify-start">
        <span>{{ t('edit.batchEdit.operationContent') }}</span>
        <label>
          {{ t('edit.batchEdit.fieldNameLabel') }}<br>
          <input v-model="operateKey" class="rounded" type="text">
        </label>
        <label>
          <span v-if="operation === 'add'">{{ t('edit.batchEdit.addValueLabel') }}</span>
          <span v-else-if="operation === 'normalize'">{{ t('edit.batchEdit.normalizeValueLabel') }}</span>
          <br>
          <input v-if="operation === 'add' || operation === 'normalize'" v-model="operateValue" type="text">
        </label>
      </div>
      <div>
        <Button class="my-4 bg-blue-500 hover:bg-blue-600" @click="operateBatch">
          {{ t('edit.batchEdit.executeButton') }}
        </Button>
      </div>
    </div>
  </div>
</template>
