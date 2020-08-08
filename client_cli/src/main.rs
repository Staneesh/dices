use client_lib::ServerConnector;
use common::MessageFromServer;
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    println!("Welcome to DAJSES");

    let mut server_connector = ServerConnector::new("127.0.0.1:3000").unwrap();

    loop {
        let wait_result = server_connector.wait_for_message();
        let task = wait_result.unwrap();

        match task {
            MessageFromServer::Init {
                players,
                round_number,
                your_dices,
            } => {
                println!("Init!");
            }

            MessageFromServer::YourMove { username } => {
                println!("YourMove!");
            }

            MessageFromServer::RoundEnd { loser } => {
                println!("RoundEnd!");
            }

            MessageFromServer::GameEnd { winner } => {
                println!("GameEnd!");
            }
        }
    }
}
