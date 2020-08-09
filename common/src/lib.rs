use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum MessageFromServer
{
    LoginResponse(bool),
    Init {
        players: Vec<String>,
        round_number: u64,
        your_dices: Vec<u8>
    },
    YourMove {
        username: String
    },
    RoundEnd {
        loser: String
    },
    GameEnd {
        winner: String
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageFromClient
{
    Login {
        username: String,
        password: String
    },
    JoinRoom {
        room_id: u64
    },
    StartGame,
    Bet {
        dices_count: u64,
        number_on_dice: u64
    },
    Check
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
