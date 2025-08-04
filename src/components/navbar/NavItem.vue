<script setup lang="ts">
defineProps<{
  to: string
  title?: string
  disabled?: boolean
  icon: string
}>()

defineEmits(['click'])
</script>

<template>
  <RouterLink
    :to="to"
    :title="title"
    :class="{ 'pointer-events-none opacity-50': disabled }"
    class="transition-colors transition-duration-300 transition-ease-DEFAULT hover:color-blue-500"
    @click="$emit('click')"
  >
    <div :class="icon" />
  </RouterLink>
</template>
