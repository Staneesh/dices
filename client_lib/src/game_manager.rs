use super::server_connector::ServerConnector;

pub struct GameManager
{
    server_connector: ServerConnector
}

impl GameManager
{
    fn new(server_address: &str) -> Result<GameManager, Box<dyn std::error::Error>>
    {
        Ok(GameManager {
            server_connector: ServerConnector::new(server_address) ?
        })
    }

    fn submit_bet(&mut self, dices_count: u64, number_on_dice: u64)
    {

    }

    fn submit_check(&mut self)
    {

    }
}