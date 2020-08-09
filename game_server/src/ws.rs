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

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::doc;

use common::{MessageFromClient, MessageFromServer};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Tx>>>;

fn parse_message(ws_message: Message) -> Result<MessageFromClient, ()>
{
    if let Message::Text(message_str) = ws_message 
    {
        match serde_json::from_str::<MessageFromClient>(&message_str) {
            Ok(message_from_client) => return Ok(message_from_client),
            Err(_) => return Err(())
        };
    } 
    else 
    {
        return Err(());
    }
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr)
{
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    // Get a handle to a database.
    let db = client.database("mydb");

    let collection = db.collection("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await.unwrap();

    println!("Incoming TCP connection from: {}", addr);

    let mut ws_stream = async_tungstenite::tokio::accept_async(raw_stream).await.unwrap();
    println!("WebSocket connection established: {}", addr);

    // Login user
    let mut user: String = String::from("unknown");

    let login_message = parse_message(ws_stream.next().await.unwrap().unwrap()).unwrap();
    if let MessageFromClient::Login {username, password} = login_message {
        println!("Received login request from {} with password {}", &username, &password);
        user = username;
    }
    else {
        panic!();
    }

    let (tx, mut rx) = unbounded_channel();

    peer_map.lock().await.insert(user.clone(), tx);

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

    peer_map.lock().await.remove(&user);
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
