import {socket} from "@/socket.ts";
import {ref} from "vue";

const isConnected = ref(false);
const messages = ref<string[]>([])
const logs = ref<string[]>([])

export function useSocket() {
    const init = () => {
        if (socket.hasListeners("pong")) return;
        logs.value.push('> Initialize connection sequence...')
        socket.on("connect", () => {
            isConnected.value = true;
            console.log("Connected with ID:", socket.id);
        });
        socket.on("disconnect", () => {
            isConnected.value = false;
            console.log("Disconnected from server");
        });
        socket.on("connect_error", (err) => {
            console.log("Connection error:", err);
        });
        socket.on("pong", (data) => {
            console.log("Received pong:", data);
            messages.value.push(`Server said: ${data}`);
        });
    }

    const sendPing = () => {
        console.log("Sending ping...");
        socket.emit("ping", "Hello from Vue!");
        console.log("Ping sent!");
    }

    return {isConnected, messages, logs, init, sendPing}
}