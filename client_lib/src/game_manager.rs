use super::server_connector::ServerConnector;
use common::{MessageFromClient, MessageFromServer};

pub struct GameManager
{
    server_connector: ServerConnector
}

impl GameManager
{
    pub fn new(server_address: &str) -> Result<GameManager, Box<dyn std::error::Error>>
    {
        Ok(GameManager {
            server_connector: ServerConnector::new(server_address) ?
        })
    }

    pub fn login(&mut self, username: &str, password: &str) 
    -> Result<bool, Box<dyn std::error::Error>>
    {
        let login_message = MessageFromClient::Login
        { 
            username: String::from(username), 
            password: String::from(password) 
        };

        self.server_connector.send_message(login_message) ?;

        let response_msg: MessageFromServer = self.server_connector.wait_for_message() ?;

        if let MessageFromServer::LoginResponse(login_response) = response_msg {
            return Ok(login_response)
        }
        else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, 
                       "Server should respond with login response but did something else")));
        }  
    }

    pub fn submit_bet(&mut self, dices_count: u64, number_on_dice: u64)
    {

    }

    pub fn submit_check(&mut self)
    {

    }
}