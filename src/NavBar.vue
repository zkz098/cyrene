<script setup lang="ts">
import { ask, message } from '@tauri-apps/plugin-dialog'
import { useFilesStore } from './stores/useFilesStore'
import { writeMultipleFrontmatter } from './utils/tauri'

const filesStore = useFilesStore()

async function saveFiles() {
  const temp = {} as Record<string, Record<string, unknown>>

  filesStore.getFileAbsolutePathList().forEach((key) => {
    if (filesStore.files[key].modified === false)
      return
    temp[key] = filesStore.files[key].frontmatter
  })

  const saveOrNot = await ask('你所作的更改将立刻写入到文件系统中，且不可撤销，是否继续？', { title: '确认保存', kind: 'warning' })

  if (saveOrNot) {
    const result = await writeMultipleFrontmatter(temp)
    if (Object.values(result).every(v => v)) {
      await message('所有文件已成功保存', { title: '保存成功', kind: 'info' })
    }
    else {
      await message('部分文件保存失败，请检查日志', { title: '保存失败', kind: 'error' })
    }
  }
}
</script>

<template>
  <nav>
    <ul class="fixed m-0 h-90vh flex flex-col list-none items-center justify-start gap-8 rounded-2xl bg-gray-100 p-4 pt-8 text-8 color-gray-700">
      <li>
        <RouterLink
          to="/"
        >
          <div class="i-ri-home-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/files"
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
        >
          <div class="i-ri-node-tree" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/edit"
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
        >
          <div class="i-ri-edit-box-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/export"
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
        >
          <div class="i-ri-export-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/settings">
          <div class="i-ri-settings-2-line" />
        </RouterLink>
      </li>
      <li>
        <div
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
          @click="saveFiles"
        >
          <div class="i-ri-save-2-line" />
        </div>
      </li>
    </ul>
  </nav>
</template>

<style lang="css" scoped>
a {
  color: black;
}
</style>
