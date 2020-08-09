use client_lib::game_manager::GameManager;
use client_lib::server_connector::ServerConnector;
use common::MessageFromServer;
use std::io;

fn main() {
    println!("Welcome to DAJSES");

    let mut server_connector = ServerConnector::new("127.0.0.1:2137").unwrap();

    let mut game_manager = GameManager::new("127.0.0.1:2137").unwrap();
    println!("{:?}", game_manager.login("User", "Pass"));

    loop {
        let wait_result = server_connector.wait_for_message();
        let task = wait_result.unwrap();

        match task {
            MessageFromServer::Init {
                players,
                round_number,
                your_dices,
            } => {
                println!("Welcome to round {:?}.", round_number);
                println!("Players at the table: {:?}.", players);
                println!("Your dices: {:?}", your_dices);
            }

            MessageFromServer::YourMove { username } => {
                println!("Your move!");

                let mut correct_move = 0;
                let mut pick = String::new();
                let mut picked_num = 0;

                while correct_move == 0 {
                    println!("Bet - 0 , Check - 1. What is Your move?");

                    if let Ok(r1) = io::stdin().read_line(&mut pick) {
                        if let Ok(number) = pick.parse::<i32>() {
                            if number == 0 || number == 1 {
                                correct_move = 1;
                                picked_num = number;
                                continue;
                            }
                        }
                    }

                    println!("Wrong input! Try again, please.");
                }

                assert!(correct_move == 1);

                if picked_num == 0 {
                    let mut correct_bet = 0;

                    while correct_bet == 0 {
                        println!("Place Your bet! (f.ex. 3x5)");
                        let mut bet = String::new();
                        if let Ok(r1) = io::stdin().read_line(&mut bet) {
                            if bet.len() == 3 {
                                let a_res = bet[0..1].parse::<u64>();
                                let b = &bet[1..2];
                                let c_res = bet[1..3].parse::<u64>();
                                if a_res.is_ok() && a_res.is_ok() {
                                    let a = a_res.unwrap();
                                    let c = c_res.unwrap();

                                    if a <= 5 {
                                        if b == "x" {
                                            if c <= 6 {
                                                //correct!
                                                correct_bet = 1;
                                                game_manager.submit_bet(a, c);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        println!("Wrong input! Try again, please.");
                    }
                } else {
                    println!("You checked!");
                    game_manager.submit_check();
                }
            }

            MessageFromServer::RoundEnd { loser } => {
                println!("Round ended - {} lost!", loser);
            }

            MessageFromServer::GameEnd { winner } => {
                println!("Game ended - {} won!", winner);
            }
            MessageFromServer::LoginResponse(_) => {}
        }
    }
}
