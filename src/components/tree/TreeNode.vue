<script setup>
defineProps({
  node: Object,
})
</script>

<template>
  <li>
    <div>
      <span v-if="node.type === 'folder'">📁 {{ node.name }}</span>
      <span v-else>📄 {{ node.name }}</span>
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
