mod traits;

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
    type Incoming = Self;
    type Outcoming = Outcoming;
    type Connection = Server;
    type Error = ws::Error;

    fn handle(connection: &mut Self::Connection, message: Self::Incoming) -> Result<Self::Outcoming, Self::Error> {
        match message {

            Incoming::BecomeCaptain => {
                if connection.spaceship.is_some() {
                    Ok(Outcoming::AlreadyCrewed)
                } else {
                    let id = Uuid::new_v4();
                    let spaceship = Spaceship {
                        id,
                        crew: vec![connection.sender.clone()]
                    };

                    println!("new spaceship; id={}", id);

                    connection.spaceship = Some(spaceship);

                    Ok(Outcoming::NewCaptain {
                        id
                    })
                }
            }

            Incoming::LeaveCrew => {
                if let Some(ref mut spaceship) = connection.spaceship {
                    let token = connection.sender.token();
                    let index = spaceship.crew.iter()
                        .position(|member| member.token() == token)
                        .unwrap();

                    spaceship.crew.remove(index);

                    connection.spaceship = None;

                    Ok(Outcoming::LeaveCrew)
                } else {
                    Ok(Outcoming::NotCrewed)
                }
            }

        }
    }
}
