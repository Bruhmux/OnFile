import { ref, reactive } from 'vue';
import type {
  Room,
  CreateRoomResult,
  JoinRoomResult,
  GameCredentials,
  WsMessage,
  ClientMessage,
  DrawDiscoveryPayload,
  Suspect,
  Weapon,
  Location,
  Category,
} from '../types';

const rooms = ref<Room[]>([]);
const credentials = ref<GameCredentials | null>(null);
const wsConnected = ref(false);
const messages = reactive<WsMessage[]>([]);
const pendingDiscovery = ref<DrawDiscoveryPayload | null>(null);
const discoveryStep = ref<'choose-category' | 'pick-file' | 'pick-file-again' | null>(null);
const activeCategory = ref<Category | null>(null);
const disabledFiles = ref<number[]>([]);
const cardCategories = ref<Category[]>([]);
let ws: WebSocket | null = null;

export function useGame() {
  async function fetchRooms(): Promise<void> {
    try {
      const res = await fetch('/api/rooms');
      if (res.ok) {
        rooms.value = await res.json();
      }
    } catch (e) {
      console.error('Failed to fetch rooms:', e);
    }
  }

  async function createRoom(displayName: string): Promise<boolean> {
    try {
      const res = await fetch('/api/rooms', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ display_name: displayName }),
      });
      if (res.ok) {
        const data: CreateRoomResult = await res.json();
        credentials.value = {
          roomId: data.room_id,
          userId: data.user_id,
          connectionToken: data.connection_token,
          displayName,
        };
        return true;
      }
      return false;
    } catch (e) {
      console.error('Failed to create room:', e);
      return false;
    }
  }

  async function joinRoom(roomId: string, displayName: string): Promise<boolean> {
    try {
      const res = await fetch(`/api/rooms/${roomId}/join`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ display_name: displayName }),
      });
      if (res.ok) {
        const data: JoinRoomResult = await res.json();
        credentials.value = {
          roomId: roomId.toUpperCase(),
          userId: data.user_id,
          connectionToken: data.connection_token,
          displayName,
        };
        return true;
      }
      return false;
    } catch (e) {
      console.error('Failed to join room:', e);
      return false;
    }
  }

  async function initFiles(amount: number): Promise<boolean> {
    if (!credentials.value) return false;
    try {
      const res = await fetch(`/api/rooms/${credentials.value.roomId}/files`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount }),
      });
      return res.ok;
    } catch (e) {
      console.error('Failed to init files:', e);
      return false;
    }
  }

  function leaveRoom(): void {
    disconnectWebSocket();
    credentials.value = null;
    messages.length = 0;
    clearPendingDiscovery();
  }

  function connectWebSocket(): void {
    if (!credentials.value) return;
    if (ws && ws.readyState === WebSocket.OPEN) return;

    const { roomId, connectionToken } = credentials.value;
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/api/ws/${roomId}?token=${connectionToken}`;

    ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      wsConnected.value = true;
      addMessage('received', { type: 'System', message: 'WebSocket connected' });
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        addMessage('received', data);
        if (data.type === 'DrawDiscovery') {
          const payload = data.payload as DrawDiscoveryPayload;
          pendingDiscovery.value = payload;

          if (payload.card === 'Wild') {
            discoveryStep.value = 'pick-file';
            activeCategory.value = null;
            cardCategories.value = [];
          } else if ('Same' in payload.card) {
            discoveryStep.value = 'pick-file';
            activeCategory.value = payload.card.Same;
            cardCategories.value = [payload.card.Same];
          } else if ('Different' in payload.card) {
            discoveryStep.value = 'choose-category';
            activeCategory.value = null;
            cardCategories.value = payload.card.Different;
          }
          disabledFiles.value = [];
        }
      } catch {
        addMessage('received', { raw: event.data });
      }
    };

    ws.onclose = () => {
      wsConnected.value = false;
      ws = null;
      addMessage('received', { type: 'System', message: 'WebSocket disconnected' });
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      addMessage('received', { type: 'Error', message: 'WebSocket error occurred' });
    };
  }

  function disconnectWebSocket(): void {
    if (ws) {
      ws.close();
      ws = null;
    }
    wsConnected.value = false;
  }

  function sendMessage(msg: ClientMessage): void {
    if (!ws || ws.readyState !== WebSocket.OPEN) {
      addMessage('received', { type: 'Error', message: 'WebSocket not connected' });
      return;
    }
    addMessage('sent', msg);
    ws.send(JSON.stringify(msg));
  }

  function sendChat(message: string): void {
    sendMessage({ type: 'Chat', payload: message });
  }

  function sendDrawDiscovery(): void {
    sendMessage({ type: 'DrawDiscovery' });
  }

  function sendGuess(suspect: Suspect, weapon: Weapon, location: Location): void {
    sendMessage({
      type: 'Guess',
      payload: { suspect, weapon, location },
    });
  }

  function sendChooseFile(discoveryId: string, fileIdx: number, category: Category): void {
    sendMessage({ type: 'ChooseFile', payload: { discovery_id: discoveryId, file_idx: fileIdx, category } });
  }

  function sendInitFiles(amount: number): void {
    sendMessage({ type: 'InitFiles', payload: { amount } });
  }

  function clearPendingDiscovery(): void {
    pendingDiscovery.value = null;
    discoveryStep.value = null;
    activeCategory.value = null;
    disabledFiles.value = [];
    cardCategories.value = [];
  }

  function discoveryPickCategory(cat: Category): void {
    activeCategory.value = cat;
    discoveryStep.value = 'pick-file';
  }

  function discoveryPickFile(fileIdx: number, category?: Category): void {
    const cat = category ?? activeCategory.value;
    if (!pendingDiscovery.value || !cat) return;
    sendChooseFile(pendingDiscovery.value.discovery_id, fileIdx, cat);
    disabledFiles.value.push(fileIdx);

    if (discoveryStep.value === 'pick-file-again') {
      clearPendingDiscovery();
    } else if (cardCategories.value.length === 2) {
      const other = cardCategories.value.find(c => c !== cat) ?? null;
      activeCategory.value = other;
      discoveryStep.value = 'pick-file-again';
    } else if (cardCategories.value.length === 1) {
      discoveryStep.value = 'pick-file-again';
    } else {
      clearPendingDiscovery();
    }
  }

  function cancelDiscovery(): void {
    clearPendingDiscovery();
  }

  function sendPlaceClue(
    x_category: Category,
    x_idx: number,
    y_category: Category,
    y_idx: number,
    is_true: boolean
  ): void {
    sendMessage({
      type: 'PlaceClue',
      payload: { x_category, x_idx, y_category, y_idx, is_true },
    });
  }

  function addMessage(direction: 'sent' | 'received', data: unknown): void {
    messages.push({
      timestamp: new Date().toISOString(),
      direction,
      data,
    });
  }

  function clearMessages(): void {
    messages.length = 0;
  }

  return {
    rooms,
    credentials,
    wsConnected,
    messages,
    pendingDiscovery,
    discoveryStep,
    activeCategory,
    disabledFiles,
    cardCategories,
    fetchRooms,
    createRoom,
    joinRoom,
    leaveRoom,
    initFiles,
    connectWebSocket,
    disconnectWebSocket,
    sendChat,
    sendDrawDiscovery,
    sendChooseFile,
    sendInitFiles,
    clearPendingDiscovery,
    discoveryPickCategory,
    discoveryPickFile,
    cancelDiscovery,
    sendGuess,
    sendPlaceClue,
    clearMessages,
  };
}
