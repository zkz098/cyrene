<script setup lang="ts">
import { ref, watch } from 'vue'
import { useFilesStore } from '../stores/useFilesStore'
import { buildFileTree } from '../utils/buildFileTree'

import { getRelativePath } from '../utils/getRelativePath'
import { readAndParseYamlFrontmatter } from '../utils/tauri'
import JsonEditor from './tree/jsonEditor.vue'
import TreeNode from './tree/TreeNode.vue'

const filesStore = useFilesStore()

if (!filesStore.ready.fileTree) {
  filesStore.filetree = buildFileTree(filesStore.getFileRelativePathList())
  filesStore.ready.fileTree = true
}

const jsonContent = ref('{\n  "name": "example",\n  "value": 123\n}')

watch(() => filesStore.currentAccessPath, async (newPath) => {
  if (newPath) {
    if (filesStore.files[newPath]?.frontmatter) {
      jsonContent.value = JSON.stringify(filesStore.files[newPath].frontmatter, null, 2)
    }
    else {
      if (!filesStore.files[newPath]) {
        filesStore.files[newPath] = { frontmatter: {}, relativePath: getRelativePath(newPath, filesStore.basePath), modified: false }
      }
      filesStore.files[newPath].frontmatter = await readAndParseYamlFrontmatter(newPath)
      jsonContent.value = JSON.stringify(filesStore.files[newPath].frontmatter, null, 2)
    }
  }
}, { immediate: true })
</script>

<template>
  <div v-if="!filesStore.ready.fileTree" class="flex flex-col items-center justify-center p-4">
    <h1 class="text-2xl">
      文件树加载中...
    </h1>
    <p>请稍等片刻，文件树正在准备中。对于一些大型站点，此过程可能需要几分钟甚至更久</p>
  </div>
  <div v-else class="flex flex-col items-start justify-start p-4">
    <div class="mb-4 w-full flex flex-row items-center justify-between">
      <ul class="h-90vh min-w-30vw flex flex-col list-none items-start justify-start gap-2 overflow-scroll">
        <TreeNode v-for="(node, index) in filesStore.filetree" :key="index" :node="node" />
      </ul>
      <div class="h-100% w-full flex items-start justify-start">
        <JsonEditor
          v-model="jsonContent"
          :readonly="false"
          theme="github-light"
          placeholder="请输入JSON内容..."
        />
      </div>
    </div>
  </div>
</template>
