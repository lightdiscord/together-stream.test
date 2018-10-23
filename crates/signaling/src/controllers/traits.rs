use ws::{ self, Message, Sender };
use serde_json;
use std::io;
use serde::{ Deserialize, Serialize };

pub trait Handler<'de> {
    type Incoming: Deserialize<'de>;
    type Outcoming: Serialize;

    type Connection: AsSender;
    type Error: From<ws::Error> + From<io::Error>;

    fn handle(connection: &mut Self::Connection, message: Self::Incoming) -> Result<Self::Outcoming, Self::Error>;

    fn on_message(connection: &mut Self::Connection, message: &'de Message) -> Result<(), Self::Error> {
        let message = message.as_text();
        let action: Self::Incoming = serde_json::from_str(message?)
            .map_err(io::Error::from)?;

        let result = Self::handle(connection, action)?;
        let result = serde_json::to_string(&result)
            .map_err(io::Error::from)?;

        Ok(connection.sender().send(result)?)
    }
}

pub trait AsSender {
    fn sender(&self) -> &Sender;
}
