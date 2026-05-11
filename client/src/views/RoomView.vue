<script setup lang="ts">
import { ref } from 'vue';
import { useGame } from '../composables/useGame';
import {
  SUSPECTS,
  WEAPONS,
  LOCATIONS,
  CATEGORIES,
  INDEX_OPTIONS,
  type Suspect,
  type Weapon,
  type Location,
  type Category,
} from '../types';

const emit = defineEmits<{
  (e: 'leave'): void;
}>();

const {
  credentials,
  wsConnected,
  messages,
  pendingDiscovery,
  connectWebSocket,
  disconnectWebSocket,
  sendChat,
  sendDrawDiscovery,
  sendGuess,
  sendPlaceClue,
  sendChooseFile,
  sendInitFiles,
  clearPendingDiscovery,
  clearMessages,
  leaveRoom,
} = useGame();

const chatInput = ref('');
const selectedSuspect = ref<Suspect>(SUSPECTS[0]!.value);
const selectedWeapon = ref<Weapon>(WEAPONS[0]!.value);
const selectedLocation = ref<Location>(LOCATIONS[0]!.value);
const clueXCategory = ref<Category>(CATEGORIES[0]!.value);
const clueXIdx = ref(0);
const clueYCategory = ref<Category>(CATEGORIES[1]!.value);
const clueYIdx = ref(0);
const clueIsTrue = ref(true);

const fileAmount = ref(4);

const chosenFileIdx = ref(0);
const chosenCategory = ref<Category>('suspect');

function handleConnect(): void {
  connectWebSocket();
}

function handleDisconnect(): void {
  disconnectWebSocket();
}

function handleSendChat(): void {
  if (!chatInput.value.trim()) return;
  sendChat(chatInput.value.trim());
  chatInput.value = '';
}

function handleDrawDiscovery(): void {
  sendDrawDiscovery();
}

function handleGuess(): void {
  sendGuess(selectedSuspect.value, selectedWeapon.value, selectedLocation.value);
}

function handlePlaceClue(): void {
  sendPlaceClue(
    clueXCategory.value,
    clueXIdx.value,
    clueYCategory.value,
    clueYIdx.value,
    clueIsTrue.value
  );
}

function handleInitFiles(): void {
  sendInitFiles(fileAmount.value);
}

function handleChooseFile(): void {
  if (!pendingDiscovery.value) return;
  sendChooseFile(pendingDiscovery.value.discovery_id, chosenFileIdx.value, chosenCategory.value);
  clearPendingDiscovery();
}

function handleCancelPick(): void {
  clearPendingDiscovery();
}

function handleLeaveRoom(): void {
  leaveRoom();
  emit('leave');
}

function formatJson(data: unknown): string {
  try {
    return JSON.stringify(data, null, 2);
  } catch {
    return String(data);
  }
}

function formatTime(isoString: string): string {
  return new Date(isoString).toLocaleTimeString();
}

function getIndexLabel(category: Category, idx: number): string {
  const list = category === 'suspect' ? SUSPECTS : category === 'weapon' ? WEAPONS : LOCATIONS;
  return list[idx]?.label || String(idx);
}
</script>

<template>
  <div class="h-screen flex flex-col font-sans">
    <header class="flex justify-between items-center px-6 py-3 bg-gray-900 text-white">
      <div class="flex items-center gap-4">
        <h1 class="text-lg font-semibold">Room: {{ credentials?.roomId }}</h1>
        <span class="text-sm text-indigo-300">Player: {{ credentials?.displayName }}</span>
      </div>
      <div class="flex items-center gap-3">
        <span class="text-sm" :class="wsConnected ? 'text-green-400' : 'text-gray-400'">
          {{ wsConnected ? '● Connected' : '○ Disconnected' }}
        </span>
        <button
          v-if="!wsConnected"
          @click="handleConnect"
          class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded hover:bg-indigo-700 transition-colors"
        >
          Connect
        </button>
        <button
          v-else
          @click="handleDisconnect"
          class="px-3 py-1.5 text-sm bg-gray-700 text-gray-200 rounded hover:bg-gray-600 transition-colors"
        >
          Disconnect
        </button>
        <button
          @click="handleLeaveRoom"
          class="px-3 py-1.5 text-sm bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
        >
          Leave Room
        </button>
      </div>
    </header>

    <div v-if="pendingDiscovery" class="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
      <div class="bg-white rounded-xl shadow-xl p-6 w-80 max-w-full">
        <h3 class="text-lg font-semibold text-gray-900 mb-1">Pick a File</h3>
        <p class="text-sm text-gray-500 mb-4">Choose a file to examine (0–{{ pendingDiscovery.files - 1 }})</p>

        <label class="block text-xs font-medium text-gray-600 mb-1">File Index</label>
        <select v-model.number="chosenFileIdx" class="w-full mb-3 px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500">
          <option v-for="i in pendingDiscovery.files" :key="i - 1" :value="i - 1">
            File {{ i }}
          </option>
        </select>

        <label class="block text-xs font-medium text-gray-600 mb-1">Category</label>
        <select v-model="chosenCategory" class="w-full mb-4 px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500">
          <option v-for="c in CATEGORIES" :key="c.value" :value="c.value">{{ c.label }}</option>
        </select>

        <div class="flex gap-2">
          <button @click="handleChooseFile" class="flex-1 px-3 py-2 bg-indigo-600 text-white text-sm font-medium rounded hover:bg-indigo-700 transition-colors">
            Examine File
          </button>
          <button @click="handleCancelPick" class="px-3 py-2 bg-gray-200 text-gray-700 text-sm font-medium rounded hover:bg-gray-300 transition-colors">
            Cancel
          </button>
        </div>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <div class="w-96 p-4 bg-gray-100 border-r border-gray-200 overflow-y-auto">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">WebSocket Actions</h2>

        <div class="mb-4 p-3 bg-white rounded-lg border border-gray-200">
          <h3 class="mb-2 text-xs font-medium text-gray-500 uppercase tracking-wider">Init Files</h3>
          <div class="flex gap-2 mb-2">
            <input
              v-model.number="fileAmount"
              type="number"
              min="4"
              placeholder="File count"
              class="w-20 px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
            <button
              @click="handleInitFiles"
              class="px-3 py-1.5 text-sm bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
            >
              Init Files
            </button>
          </div>
        </div>

        <div class="mb-4 p-3 bg-white rounded-lg border border-gray-200">
          <h3 class="mb-2 text-xs font-medium text-gray-500 uppercase tracking-wider">Chat</h3>
          <div class="flex gap-2">
            <input
              v-model="chatInput"
              type="text"
              placeholder="Type a message..."
              @keyup.enter="handleSendChat"
              class="flex-1 px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
            <button
              @click="handleSendChat"
              class="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded hover:bg-indigo-700 transition-colors"
            >
              Send
            </button>
          </div>
        </div>

        <div class="mb-4 p-3 bg-white rounded-lg border border-gray-200">
          <h3 class="mb-2 text-xs font-medium text-gray-500 uppercase tracking-wider">Draw Discovery</h3>
          <button
            @click="handleDrawDiscovery"
            class="w-full px-3 py-2.5 bg-indigo-600 text-white font-medium rounded hover:bg-indigo-700 transition-colors"
          >
            Draw Discovery Card
          </button>
        </div>

        <div class="mb-4 p-3 bg-white rounded-lg border border-gray-200">
          <h3 class="mb-2 text-xs font-medium text-gray-500 uppercase tracking-wider">Submit Guess (Verdict)</h3>
          <div class="space-y-2 mb-3">
            <div>
              <label class="block text-xs font-medium text-gray-600 mb-0.5">Suspect</label>
              <select
                v-model="selectedSuspect"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="s in SUSPECTS" :key="s.value" :value="s.value">
                  {{ s.label }}
                </option>
              </select>
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-600 mb-0.5">Weapon</label>
              <select
                v-model="selectedWeapon"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="w in WEAPONS" :key="w.value" :value="w.value">
                  {{ w.label }}
                </option>
              </select>
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-600 mb-0.5">Location</label>
              <select
                v-model="selectedLocation"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="l in LOCATIONS" :key="l.value" :value="l.value">
                  {{ l.label }}
                </option>
              </select>
            </div>
          </div>
          <button
            @click="handleGuess"
            class="w-full px-3 py-2 bg-indigo-600 text-white text-sm font-medium rounded hover:bg-indigo-700 transition-colors"
          >
            Submit Guess
          </button>
        </div>

        <div class="p-3 bg-white rounded-lg border border-gray-200">
          <h3 class="mb-2 text-xs font-medium text-gray-500 uppercase tracking-wider">Place Clue</h3>
          <div class="flex flex-wrap gap-2 items-end mb-3">
            <div class="flex-1 min-w-28">
              <label class="block text-xs font-medium text-gray-600 mb-0.5">X Category</label>
              <select
                v-model="clueXCategory"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="c in CATEGORIES" :key="c.value" :value="c.value">
                  {{ c.label }}
                </option>
              </select>
            </div>
            <div class="flex-1 min-w-28">
              <label class="block text-xs font-medium text-gray-600 mb-0.5">X Index</label>
              <select
                v-model="clueXIdx"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="i in INDEX_OPTIONS" :key="i" :value="i">
                  {{ i }} - {{ getIndexLabel(clueXCategory, i) }}
                </option>
              </select>
            </div>
            <div class="text-gray-400 font-bold px-1">↔</div>
            <div class="flex-1 min-w-28">
              <label class="block text-xs font-medium text-gray-600 mb-0.5">Y Category</label>
              <select
                v-model="clueYCategory"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="c in CATEGORIES" :key="c.value" :value="c.value">
                  {{ c.label }}
                </option>
              </select>
            </div>
            <div class="flex-1 min-w-28">
              <label class="block text-xs font-medium text-gray-600 mb-0.5">Y Index</label>
              <select
                v-model="clueYIdx"
                class="w-full px-2.5 py-1.5 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                <option v-for="i in INDEX_OPTIONS" :key="i" :value="i">
                  {{ i }} - {{ getIndexLabel(clueYCategory, i) }}
                </option>
              </select>
            </div>
            <div class="flex items-center pb-1.5">
              <label class="flex items-center gap-1.5 text-sm text-gray-700 cursor-pointer">
                <input type="checkbox" v-model="clueIsTrue" class="rounded" />
                Is True
              </label>
            </div>
          </div>
          <button
            @click="handlePlaceClue"
            class="w-full px-3 py-2 bg-indigo-600 text-white text-sm font-medium rounded hover:bg-indigo-700 transition-colors"
          >
            Place Clue
          </button>
        </div>
      </div>

      <div class="flex-1 flex flex-col bg-gray-50">
        <div class="flex justify-between items-center px-4 py-3 bg-white border-b border-gray-200">
          <h2 class="text-base font-medium text-gray-700">Messages ({{ messages.length }})</h2>
          <button
            @click="clearMessages"
            class="px-2.5 py-1 text-xs bg-gray-100 text-gray-600 rounded hover:bg-gray-200 transition-colors"
          >
            Clear
          </button>
        </div>
        <div class="flex-1 overflow-y-auto p-3">
          <div
            v-if="messages.length === 0"
            class="text-center text-gray-400 py-8 text-sm"
          >
            No messages yet. Connect and send an action.
          </div>
          <div v-else class="space-y-2">
            <div
              v-for="(msg, idx) in messages"
              :key="idx"
              class="rounded-lg border border-gray-200 overflow-hidden"
              :class="msg.direction === 'sent' ? 'bg-blue-50' : 'bg-white'"
            >
              <div
                class="flex justify-between px-2.5 py-1 text-xs"
                :class="msg.direction === 'sent' ? 'bg-blue-100 text-blue-800' : 'bg-gray-100 text-gray-600'"
              >
                <span class="font-semibold">{{ msg.direction.toUpperCase() }}</span>
                <span :class="msg.direction === 'sent' ? 'text-blue-600' : 'text-gray-500'">
                  {{ formatTime(msg.timestamp) }}
                </span>
              </div>
              <pre class="m-0 p-2.5 text-xs font-mono overflow-x-auto whitespace-pre-wrap break-all">
{{ formatJson(msg.data) }}</pre
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
