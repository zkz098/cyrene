<script setup lang="ts">
import { ask, message } from '@tauri-apps/plugin-dialog'
import { useI18n } from 'vue-i18n'
import { useFilesStore } from '../stores/useFilesStore'
import { writeMultipleFrontmatter } from '../utils/tauri'
import NavItem from './navbar/NavItem.vue'

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
        <NavItem
          to="/"
          :title="t('nav.home')"
          icon="i-ri-home-line"
        />
      </li>
      <li>
        <NavItem
          to="/files"
          :title="t('nav.files')"
          :disabled="!filesStore.ready.fileList"
          icon="i-ri-node-tree"
        />
      </li>
      <li>
        <NavItem
          to="/edit"
          :title="t('nav.edit')"
          :disabled="!filesStore.ready.fileList"
          icon="i-ri-edit-box-line"
        />
      </li>
      <li>
        <NavItem
          to="/export"
          :title="t('nav.export')"
          :disabled="!filesStore.ready.fileList"
          icon="i-ri-export-line"
        />
      </li>
      <li>
        <NavItem
          to="/settings"
          :title="t('nav.settings')"
          icon="i-ri-settings-2-line"
        />
      </li>
      <li>
        <NavItem
          to="#"
          :title="t('common.save')"
          :disabled="!filesStore.ready.fileList"
          icon="i-ri-save-2-line"
          @click="saveFiles"
        />
      </li>
    </ul>
  </nav>
</template>

<style lang="css" scoped>
a {
  color: black;
}
</style>
