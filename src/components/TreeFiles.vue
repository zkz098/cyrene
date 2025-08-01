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

const jsonContent = ref('{\n  "name": "这是树形视图界面",\n  "value": "你可以在此页面内编辑指定文档的frontmatter，从左侧中选择一个文件以开始"\n}')

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
      <div class="h-100% w-full flex flex-col items-start justify-start">
        <JsonEditor
          v-model="jsonContent"
          :readonly="false"
          theme="github-light"
          placeholder="请输入JSON内容..."
        />
        <button class="m-2 mt-2 rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600" @click="filesStore.files[filesStore.currentAccessPath].frontmatter = JSON.parse(jsonContent)">
          保存当前数据到内存
        </button>
      </div>
    </div>
  </div>
</template>
