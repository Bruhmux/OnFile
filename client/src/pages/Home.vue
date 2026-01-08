<script setup lang="ts">
import {Button} from "@/components/ui/button";
import {onMounted, ref} from "vue";
import {useSocket} from "@/composables/useSocket.ts";

type TerminalState = 'MENU' | 'JOINING' | 'CREATING' | 'LOBBY_CREATED'

const inputValue = ref('')
const currentState = ref<TerminalState>('MENU')
const isLoading = ref(false)

const { logs, messages, init, sendPing } = useSocket();

onMounted(() => {
  init()
})

const selectOption = (option: 'create' | 'join') => {
  if (option === 'join') {
    currentState.value = 'JOINING'
    submitJoinCode()
  } else {
    currentState.value = 'CREATING'
    runCreateSequence()
  }
}

const runCreateSequence = async () => {
  isLoading.value = true
  const steps = [
    '> Allocating memory for new universe...',
    '> Summoning NPCs...',
    '> Burying the murder weapon...',
    '> Session ID generated: XJ-99'
  ]

  for (const step of steps) {
    logs.value.push(step)
    // Random "typing" delay for effect
    await new Promise(r => setTimeout(r, 600))
  }

  isLoading.value = false
  currentState.value = 'LOBBY_CREATED'
}

const submitJoinCode = () => {
  if (!inputValue.value) return

  logs.value.push(`> Connecting to session [${inputValue.value}]...`)
  isLoading.value = true

  setTimeout(() => {
    isLoading.value = false
    logs.value.push('> Connection successful. Welcome, Detective.')
    // TODO: Router.push('/lobby')
  }, 1500)
}
</script>

<template>
  <div class="h-screen flex flex-col items-center justify-center space-y-8 w-full">
     <Button @click="sendPing">Send Ping</Button>
    <ul class="text-primary font-mono text-sm">
      <li v-for="(msg, i) in messages" :key="i">{{ msg }}</li>
    </ul>

    <div class="w-full px-4 py-2 border-b border-primary/10 select-none max-w-2xl text-primary font-mono overflow-hidden">
      <span class="text-xs tracking-widest opacity-50">OS // CASTER</span>
    </div>
    <pre class="leading-3 mb-6 opacity-70 select-none text-primary">
   __   _  _  ____  __  __    ____
  /  \ ( \( )(  __)(  )(  )  (  __)
 (  O ) )  (  ) _)  )( / (_/\ ) _)
  \__/ (_)\_)(__)  (__)\____/(____)
        </pre>
    <div class="h-100 flex flex-col justify-center w-1/4">
      <div v-for="(log, i) in logs" :key="i" class="text-sm text-primary">
        <span class="opacity-50">{{ new Date().toLocaleTimeString('en-US', {hour12: false}) }}</span>
        {{ log }}
      </div>
      <div v-if="currentState === 'MENU'" class="space-y-6 relative group mt-4">
        <button
            @click="selectOption('create')"
            class="w-full text-left p-4 border border-primary/30 hover:bg-primary/10 transition-all group flex items-center"
        >
            <span class="absolute -top-3 left-3 bg-secondary px-2 text-xs text-primary/70">
              INITIATE_HOST
            </span>
          <span class="text-primary mr-3 animate-pulse">></span>
          <span class="font-bold tracking-widest">Start a new investigation</span>
        </button>

        <div v-if="currentState === 'MENU'" class="relative group">
          <div class="absolute -top-3 left-3 bg-secondary px-2 text-xs text-primary/70">
            JOIN_EXISTING_SESSION
          </div>

          <div class="border border-primary/30 p-4 flex items-center hover:bg-primary/5 transition-colors focus-within:border-primary focus-within:bg-primary/10">
            <span class="text-primary mr-3 animate-pulse">></span>
            <input
                v-model="inputValue"
                @keydown.enter="selectOption('join')"
                :disabled="isLoading"
                type="text"
                class="bg-transparent border-none outline-none text-foreground w-full font-mono font-bold uppercase tracking-widest placeholder:text-primary/40"
                placeholder="PASTE_CODE_HERE"
                maxlength="4"
            />
            <span v-if="inputValue.length > 0" class="text-xs text-primary/50 ml-2">
                [ENTER]
              </span>
          </div>
        </div>

      </div>
      <div class="flex-1 flex flex-col">
        <div class="flex-1 overflow-y-auto space-y-2 mb-4 scrollbar-hide">

          <div v-if="currentState === 'LOBBY_CREATED'" class="mt-4 p-4 border border-green-500/30 bg-green-500/10 text-green-400">
            <p class="font-bold mb-2">SUCCESS: Session Ready</p>
            <Button class="w-full bg-green-900/50 hover:bg-green-800 text-green-100 border border-green-500/50">
              Enter Lobby
            </Button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>