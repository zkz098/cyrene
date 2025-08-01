<script setup lang="ts">
import { useFilesStore } from '../stores/useFilesStore'
import { buildFileTree } from '../utils/buildFileTree'
import TreeNode from './tree/TreeNode.vue'

const filesStore = useFilesStore()

if (!filesStore.ready.fileTree) {
  filesStore.filetree = buildFileTree(filesStore.getFileRelativePathList())
  filesStore.ready.fileTree = true
}
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
      <ul class="flex flex-col list-none items-start justify-start gap-2 overflow-scroll">
        <TreeNode v-for="(node, index) in filesStore.filetree" :key="index" :node="node" />
      </ul>
      <div class="flex items-start justify-start">
        <i />
      </div>
    </div>
  </div>
</template>
