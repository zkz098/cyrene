<script setup lang="ts">
import { ask, message } from '@tauri-apps/plugin-dialog'
import { useI18n } from 'vue-i18n'
import { useFilesStore } from './stores/useFilesStore'
import { writeMultipleFrontmatter } from './utils/tauri'

const { t } = useI18n()
const filesStore = useFilesStore()

async function saveFiles() {
  const temp = {} as Record<string, Record<string, unknown>>

  filesStore.getFileAbsolutePathList().forEach((key) => {
    if (filesStore.files[key].modified === false)
      return
    temp[key] = filesStore.files[key].frontmatter
  })

  const saveOrNot = await ask(t('common.confirmSave'), { title: t('common.confirm'), kind: 'warning' })

  if (saveOrNot) {
    const result = await writeMultipleFrontmatter(temp)
    if (Object.values(result).every(v => v)) {
      await message(t('common.saveSuccess'), { title: t('common.success'), kind: 'info' })
    }
    else {
      await message(t('common.saveError'), { title: t('common.error'), kind: 'error' })
    }
  }
}
</script>

<template>
  <nav>
    <ul class="m-0 h-90vh flex flex-col list-none items-center justify-start gap-8 rounded-2xl bg-gray-100 p-4 pt-8 text-8 color-gray-700">
      <li>
        <RouterLink
          to="/"
          :title="t('nav.home')"
        >
          <div class="i-ri-home-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/files"
          :title="t('nav.files')"
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
        >
          <div class="i-ri-node-tree" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/edit"
          :title="t('nav.edit')"
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
        >
          <div class="i-ri-edit-box-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/export"
          :title="t('nav.export')"
          :class="{ 'pointer-events-none opacity-50': !filesStore.ready.fileList }"
        >
          <div class="i-ri-export-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink
          to="/settings"
          :title="t('nav.settings')"
        >
          <div class="i-ri-settings-2-line" />
        </RouterLink>
      </li>
      <li>
        <div
          :title="t('common.save')"
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
