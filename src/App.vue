<script setup lang="ts">
import NavBar from './NavBar.vue'
</script>

<template>
  <div class="flex flex-row overflow-hidden">
    <NavBar class="w-20 flex-shrink-0 lg:w-16" />

    <RouterView class="flex-1 overflow-x-hidden overflow-y-scroll" />
  </div>
</template>
