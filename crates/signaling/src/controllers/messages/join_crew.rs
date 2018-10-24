use crate::Server;
use super::{ Incoming, Outcoming, Handler };
use ws::Error;
use uuid::Uuid;

pub struct JoinCrew {
    pub id: Uuid,
}

type Return = Result<Outcoming, ws::Error>;

impl Handler for JoinCrew {
    fn handle(&self, connection: &mut Server, _message: Incoming) -> Return {
        if connection.spaceship.is_some() {
            Ok(Outcoming::AlreadyCrewed)
        } else {
            Ok(Outcoming::NotCrewed)
        }
    }
}
