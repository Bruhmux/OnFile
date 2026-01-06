<script setup lang="ts">
import ThemeToggle from "@/components/ThemeToggle.vue";
</script>

<template>
  <header class="flex justify-end m-4 gap-2">
    <ThemeToggle />
  </header>
  <main>
    <router-view />
  </main>
</template>

<style scoped>
  main {
    font-family: system-ui, sans-serif;
    padding: 2rem;
  }
</style>