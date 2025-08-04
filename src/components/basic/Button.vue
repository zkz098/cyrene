<script setup lang="ts">
defineEmits(['click'])
</script>

<template>
  <button
    class="rounded-xl border-none bg-blue-500 px-4 py-2 text-gray-100 shadow-md transition-colors transition-duration-300 transition-ease-DEFAULT hover:bg-blue-600 hover:shadow-xl focus:outline-none"
    @click="$emit('click')"
  >
    <slot />
  </button>
</template>
