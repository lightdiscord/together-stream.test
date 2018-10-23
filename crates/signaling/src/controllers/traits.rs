use ws::{ self, Message, Sender };
use serde_json;
use std::io;
use super::{ Incoming, Outcoming, Server };

pub trait Handler<'de> {
    fn handle(connection: &mut Server, message: Incoming) -> Result<Outcoming, ws::Error>;

    fn on_message(connection: &mut Server, message: &'de Message) -> Result<(), ws::Error> {
        let message = message.as_text();
        let action: Incoming = serde_json::from_str(message?)
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
