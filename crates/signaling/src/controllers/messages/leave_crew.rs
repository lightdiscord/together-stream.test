use crate::Server;
use super::{ Incoming, Outcoming, Handler };
use ws::Error;

pub struct LeaveCrew;

impl Handler for LeaveCrew {
    fn handle(&self, connection: &mut Server, _message: Incoming) -> Result<Outcoming, Error> {
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
