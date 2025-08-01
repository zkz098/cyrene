<script setup lang="ts">

</script>

<template>
  <nav>
    <ul class="m-0 h-90vh flex flex-col list-none items-center justify-start gap-8 rounded-2xl bg-gray-100 p-4 pt-8 text-8 color-gray-700">
      <li>
        <RouterLink to="/">
          <div class="i-ri-home-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/files">
          <div class="i-ri-node-tree" />
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/edit">
          <div class="i-ri-edit-box-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/export">
          <div class="i-ri-export-line" />
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/settings">
          <div class="i-ri-settings-2-line" />
        </RouterLink>
      </li>
    </ul>
  </nav>
</template>

<style lang="css" scoped>
a {
  color: black;
}
</style>
