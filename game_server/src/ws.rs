use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::Arc,
    error::Error,
};

use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedSender},
    prelude::*,
    sync::Mutex,
};
use tokio::stream::StreamExt;
use futures::SinkExt;

use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use tokio::select;
use async_tungstenite::tungstenite::protocol::Message;
use lazy_static::lazy_static;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let mut ws_stream = async_tungstenite::tokio::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, mut rx) = unbounded_channel();

    // czekaj na token usera
    let (username, game_id): (String, u32) = match ws_stream.next().await {
        Some(Ok(msg)) => {
            // tutaj zapytanie do mongodb o to w jakiej grze jest user
            
            // zwróć username i game_id
            ("jan".to_string(), 1)
        }
        _ => { return; }
    };
    peer_map.lock().await.insert(username.clone(), tx);

    loop {
        let a = rx.next();
        let b = ws_stream.next();

        select! {
            m = a => {
                if let Some(m) = m {
                    ws_stream.send(m).await.unwrap();
                }
            }
            m = b => {
                // handle game logic
            }
        }
    }

    peer_map.lock().await.remove(&username);
}

pub async fn listen(addr: String) -> Result<(), Box<dyn Error>> {
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        task::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}
