<script setup>
import { useFilesStore } from '../../stores/useFilesStore'

const props = defineProps({
  node: Object,
})

const filesStore = useFilesStore()

function handleFileClick() {
  if (props.node.type === 'file' && props.node.path) {
    filesStore.setCurrentAccessPath(props.node.path)
  }
}
</script>

<template>
  <li>
    <div>
      <span v-if="node.type === 'folder'">📁 {{ node.name }}</span>
      <span
        v-else
        class="cursor-pointer rounded px-2 py-1 hover:bg-gray-200"
        @click="handleFileClick"
      >
        📄 {{ node.name }}
      </span>
    </div>

    <ul v-if="node.type === 'folder' && node.children?.length">
      <TreeNode
        v-for="(child, index) in node.children"
        :key="index"
        :node="child"
      />
    </ul>
  </li>
</template>

<style scoped>
ul {
  padding-left: 1em;
  list-style: none;
}
</style>
