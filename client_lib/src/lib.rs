use common::MessageFromServer;

struct ServerConnector
{


}

impl ServerConnector
{
    fn new(server_address: std::net::IpAddr) -> Result<ServerConnector, String>
    {
        unimplemented!();
    }

    fn wait_for_message() -> Result<MessageFromServer, String>
    {
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
