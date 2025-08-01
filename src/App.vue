<script setup lang="ts">
import NavBar from './NavBar.vue'
</script>

<template>
  <div class="flex flex-row overflow-hidden">
    <NavBar class="max-w-10vw flex-grow-1 lg:max-w-5vw" />

    <RouterView class="max-w-90vw flex-grow-19 overflow-x-hidden overflow-y-scroll lg:max-w-95vw" />
  </div>
</template>
