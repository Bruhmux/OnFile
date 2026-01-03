<script setup lang="ts">
import { ref, onMounted } from 'vue'

const message = ref('Loading...')

onMounted(async () => {
  const res = await fetch('/api/hello')
  const data = await res.json()
  message.value = data.message
})
</script>

<template>
<div>
  <h1>Rust + Axum + Vue</h1>
  <p>{{ message }}</p>
</div>
</template>

<style scoped>

</style>