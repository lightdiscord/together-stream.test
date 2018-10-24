mod traits;

pub mod messages;
pub use self::traits::*;

use super::messages::{ Incoming, Outcoming };
use ws::{ self, Sender };
use super::Server;

impl AsSender for Server {
    fn sender(&self) -> &Sender {
        &self.sender
    }
}

impl Handler for Incoming {
    fn handle(connection: &mut Server, message: Incoming) -> Result<Outcoming, ws::Error> {
        match message {

            Incoming::BecomeCaptain => {
                messages::Handler::handle(&messages::BecomeCaptain, connection, message)
            }

            Incoming::LeaveCrew => {
                messages::Handler::handle(&messages::LeaveCrew, connection, message)
            }

            Incoming::JoinCrew { id } => {
                let action = messages::JoinCrew { id };
                messages::Handler::handle(&action, connection, message)
            }

        }
    }
}
