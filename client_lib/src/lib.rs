use common::MessageFromServer;

pub struct ServerConnector {}

impl ServerConnector {
    pub fn new(server_address: std::net::IpAddr) -> Result<ServerConnector, String> {
        unimplemented!();
    }

    pub fn wait_for_message(&mut self) -> Result<MessageFromServer, String> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
