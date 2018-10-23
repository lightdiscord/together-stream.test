use super::{ Incoming, Outcoming, Server, Spaceship, Handler };
use uuid::Uuid;
use ws::Error;

pub struct BecomeCaptain;

impl Handler for BecomeCaptain {
    fn handle(&self, connection: &mut Server, _message: Incoming) -> Result<Outcoming, Error> {
        if connection.spaceship.is_some() {
            Ok(Outcoming::AlreadyCrewed)
        } else {
            let id = Uuid::new_v4();
            let spaceship = Spaceship {
                id,
                crew: vec![connection.sender.clone()],
            };

            println!("new spaceship; id={}", id);

            connection.spaceship = Some(spaceship);

            Ok(Outcoming::NewCaptain {
                id
            })
        }
    }
}
