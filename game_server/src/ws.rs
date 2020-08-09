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

use common::{MessageFromClient, MessageFromServer};

use mongodb::{
    Client,
    options::ClientOptions,
    bson::{doc, Bson},
    options::FindOptions,
};

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

async fn verify_user(username: &str, password: &str) -> Result<bool, Box<dyn Error>>
{
    // Connect to the database
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("game");

    // Query the documents in the collection with a filter and an option.
    let document_option = db.collection("users").find_one(doc! {"username": username}, None).await?;

    if let Some(document) = document_option {
        if let Some(user_password) = document.get("password").and_then(Bson::as_str) {
            return Ok(password == user_password);
        }
    }

    Ok(false)
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr)
{
    println!("Incoming TCP connection from: {}", addr);

    let mut ws_stream = async_tungstenite::tokio::accept_async(raw_stream).await.unwrap();
    println!("WebSocket connection established: {}", addr);

    // Login user
    let mut user: String = String::from("unknown");
    let mut login_result: bool = false;

    while login_result == false 
    {
        let login_message = parse_message(ws_stream.next().await.unwrap().unwrap()).unwrap();
        if let MessageFromClient::Login {username, password} = login_message {
            println!("Received login request from {} with password {}", &username, &password);
            
            if verify_user(&username, &password).await.unwrap() {
                login_result = true;
                user = username;
            }
        }
        else {
            panic!();
        }
    
        let login_response_msg = MessageFromServer::LoginResponse(login_result);
        ws_stream.send(Message::Text(serde_json::to_string(&login_response_msg).unwrap())).await.unwrap();
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
                let client_message: MessageFromClient = parse_message(m.unwrap().unwrap()).unwrap();
                
                
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
