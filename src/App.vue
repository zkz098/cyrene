<script setup lang="ts">
import NavBar from './components/NavBar.vue'
</script>

<template>
  <div class="flex flex-row overflow-hidden">
    <NavBar class="w-20 flex-shrink-0 lg:w-16" />

    <div class="w-full flex-1 overflow-x-hidden overflow-y-scroll">
      <RouterView />
    </div>
  </div>
</template>
