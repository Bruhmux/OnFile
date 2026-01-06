<script setup lang="ts">
import { Icon } from '@iconify/vue'
import { useColorMode } from '@vueuse/core'
import {Switch} from "@/components/ui/switch";
import {computed} from "vue";

// Pass { disableTransition: false } to enable transitions
const mode = useColorMode()
const isDarkMode = computed({
  get: () => mode.value === 'dark',
  set: (value) => { mode.value = value ? 'dark' : 'light' }
})
</script>

<template>
  <div class="relative h-[1.2rem] w-[1.2rem]">
    <Icon
        icon="radix-icons:sun"
        class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
    />

    <Icon
        icon="radix-icons:moon"
        class="absolute top-0 h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
    />
  </div>
  <Switch id="mode-toggle" v-model="isDarkMode"/>
</template>
