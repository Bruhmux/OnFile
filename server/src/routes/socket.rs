use socketioxide::extract::{Data, SocketRef};

pub async fn on_connect (socket: SocketRef) {
    println!("A user connected! ID: {}", socket.id);
    socket.on("ping", on_ping);
}

async fn on_ping (socket: SocketRef, Data::<String>(data): Data<String>) {
    println!("Received ping: {:?}", data);
    socket.emit("pong", "Hello from Rust!").ok();
}