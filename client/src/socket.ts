import { io } from "socket.io-client";

export const socket = io({
    path: "/socket.io/", // Matches the Rust configuration
});