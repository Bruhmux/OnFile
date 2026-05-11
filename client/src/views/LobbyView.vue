<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useGame } from '../composables/useGame';

const emit = defineEmits<{
  (e: 'joined'): void;
}>();

const { rooms, fetchRooms, createRoom, joinRoom } = useGame();

const displayName = ref('');
const manualRoomId = ref('');
const error = ref('');
const loading = ref(false);

onMounted(() => {
  fetchRooms();
});

async function handleRefresh(): Promise<void> {
  error.value = '';
  await fetchRooms();
}

async function handleCreateRoom(): Promise<void> {
  if (!displayName.value.trim()) {
    error.value = 'Please enter your display name';
    return;
  }
  error.value = '';
  loading.value = true;
  const success = await createRoom(displayName.value.trim());
  loading.value = false;
  if (success) {
    emit('joined');
  } else {
    error.value = 'Failed to create room';
  }
}

async function handleJoinRoom(roomId?: string): Promise<void> {
  const targetRoomId = roomId || manualRoomId.value.trim();
  if (!displayName.value.trim()) {
    error.value = 'Please enter your display name';
    return;
  }
  if (!targetRoomId) {
    error.value = 'Please enter or select a room';
    return;
  }
  error.value = '';
  loading.value = true;
  const success = await joinRoom(targetRoomId, displayName.value.trim());
  loading.value = false;
  if (success) {
    emit('joined');
  } else {
    error.value = `Failed to join room "${targetRoomId}". Room may not exist.`;
  }
}

function formatDate(isoString: string): string {
  return new Date(isoString).toLocaleString();
}
</script>

<template>
  <div class="min-h-screen bg-gray-50 font-sans">
    <div class="max-w-2xl mx-auto px-4 py-8">
      <h1 class="text-3xl font-bold text-gray-900 mb-1">Crypts & Clues</h1>
      <h2 class="text-lg text-gray-600 mb-6">Game Lobby</h2>

      <div v-if="error" class="mb-4 p-3 bg-red-100 text-red-700 rounded-lg text-sm">
        {{ error }}
      </div>

      <div class="mb-6">
        <label for="displayName" class="block text-sm font-semibold text-gray-700 mb-1.5">
          Your Display Name
        </label>
        <input
          id="displayName"
          v-model="displayName"
          type="text"
          placeholder="Enter your name"
          :disabled="loading"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent disabled:bg-gray-100 disabled:cursor-not-allowed"
        />
      </div>

      <div class="mb-6 p-4 bg-gray-100 rounded-xl">
        <div class="flex justify-between items-center mb-3">
          <h3 class="text-base font-medium text-gray-800">Available Rooms</h3>
          <button
            @click="handleRefresh"
            :disabled="loading"
            class="px-3 py-1.5 text-sm bg-gray-200 text-gray-700 rounded hover:bg-gray-300 disabled:opacity-50 transition-colors"
          >
            Refresh
          </button>
        </div>

        <div v-if="rooms.length === 0" class="text-center text-gray-500 py-4">
          No rooms available. Create one below!
        </div>

        <div v-else class="space-y-2">
          <div
            v-for="room in rooms"
            :key="room.id"
            class="p-3 bg-white rounded-lg border border-gray-200 cursor-pointer hover:border-indigo-500 hover:shadow-sm transition-all"
            @click="handleJoinRoom(room.id)"
          >
            <div class="font-semibold text-gray-900">{{ room.display_name }}</div>
            <div class="flex gap-4 text-xs text-gray-500 mt-1">
              <span>Code: {{ room.id }}</span>
              <span>{{ formatDate(room.created_at) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="mb-6 p-4 bg-gray-100 rounded-xl">
        <h3 class="text-base font-medium text-gray-800 mb-3">Or Enter Room Code</h3>
        <div class="flex gap-2">
          <input
            v-model="manualRoomId"
            type="text"
            placeholder="Enter room code (e.g., ABCDE)"
            :disabled="loading"
            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:bg-gray-100"
          />
          <button
            @click="handleJoinRoom()"
            :disabled="loading"
            class="px-4 py-2 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors"
          >
            {{ loading ? 'Joining...' : 'Join Room' }}
          </button>
        </div>
      </div>

      <div class="p-4 bg-gray-100 rounded-xl">
        <h3 class="text-base font-medium text-gray-800 mb-3">Create New Room</h3>
        <button
          @click="handleCreateRoom"
          :disabled="loading"
          class="w-full px-4 py-3 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors"
        >
          {{ loading ? 'Creating...' : 'Create New Room' }}
        </button>
      </div>
    </div>
  </div>
</template>
