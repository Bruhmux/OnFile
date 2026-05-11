<script setup lang="ts">
import LobbyView from './views/LobbyView.vue';
import RoomView from './views/RoomView.vue';
import { useGame } from './composables/useGame';

const { credentials } = useGame();
</script>

<template>
  <LobbyView v-if="!credentials" @joined="() => {}" />
  <RoomView v-else @leave="() => {}" />
</template>

<style>
* {
  box-sizing: border-box;
}

html,
body,
#app {
  margin: 0;
  padding: 0;
  height: 100%;
}
</style>
