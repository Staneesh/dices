use common::{MessageFromClient, MessageFromServer};
use tungstenite::client::AutoStream;
use tungstenite::protocol::WebSocket;

pub struct ServerConnector {
    connection_stream: WebSocket<AutoStream>,
}

impl ServerConnector {
    pub fn new(server_address: &str) -> Result<ServerConnector, Box<dyn std::error::Error>> {
        let (websock_stream, _response) =
            tungstenite::client::connect(String::from("ws://") + server_address + "/")?;

        return Ok(ServerConnector {
            connection_stream: websock_stream,
        });
    }

    pub fn wait_for_message(&mut self) -> Result<MessageFromServer, tungstenite::error::Error> {
        loop {
            let websock_message = self.connection_stream.read_message()?;

            if let tungstenite::Message::Text(message_text) = websock_message {
                if let Ok(message) = serde_json::from_str::<MessageFromServer>(&message_text) {
                    return Ok(message);
                }
            }
        }
    }

    pub fn send_message(
        &mut self,
        message: MessageFromClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let message_json_text: String = serde_json::to_string(&message)?;
        let webscok_message = tungstenite::Message::Text(message_json_text);

        self.connection_stream.write_message(webscok_message)?;

        Ok(())
    }
}
