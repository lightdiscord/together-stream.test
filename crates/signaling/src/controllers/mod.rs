mod traits;

pub mod messages;
pub use self::traits::*;

use super::messages::{ Incoming, Outcoming };
use ws::{ self, Sender };
use uuid::Uuid;

pub struct Server {
    pub sender: Sender,
    pub spaceship: Option<Spaceship>,
}

impl AsSender for Server {
    fn sender(&self) -> &Sender {
        &self.sender
    }
}

pub struct Spaceship {
    pub id: Uuid,
    pub crew: Vec<Sender>,
}

impl<'de> Handler<'de> for Incoming {
    fn handle(connection: &mut Server, message: Incoming) -> Result<Outcoming, ws::Error> {
        match message {

            Incoming::BecomeCaptain => {
                messages::Handler::handle(&messages::BecomeCaptain, connection, message)
            }

            Incoming::LeaveCrew => {
                messages::Handler::handle(&messages::LeaveCrew, connection, message)
            }

        }
    }
}
