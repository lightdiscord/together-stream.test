extern crate ws;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use ws::{ Sender, Handler, Request, Response };
use uuid::Uuid;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MRequest {
    BecomeCaptain,
    LeaveCrew,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MResponse {
    Crewed,
    LeaveCrew,
    NewCaptain {
        id: Uuid
    },
    NotCrewed,
}

pub struct Server {
    pub sender: Sender,
    pub spaceship: Option<Spaceship>,
}

pub struct Spaceship {
    pub id: Uuid,
    pub crew: Vec<Sender>
}

impl Handler for Server {
    fn on_request(&mut self, request: &Request) -> ws::Result<(Response)> {
        match request.resource() {
            "/ws" => Response::from_request(request),
            "/" => Ok(Response::new(200, "OK", b"You should go to /ws".to_vec())),
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {

        let action: MRequest = serde_json::from_str(message.as_text()?)
            .map_err(io::Error::from)?;

        println!("{:?}", action);

        match action {
            MRequest::BecomeCaptain => if self.spaceship.is_some() {
                let response = serde_json::to_string(&MResponse::Crewed)
                    .map_err(io::Error::from)?;

                self.sender.send(response)
            } else {
                let id = Uuid::new_v4();
                let spaceship = Spaceship {
                    id,
                    crew: vec![self.sender.clone()]
                };

                println!("new spaceship; id={}", id);

                self.spaceship = Some(spaceship);

                let response = serde_json::to_string(&MResponse::NewCaptain { id })
                    .map_err(io::Error::from)?;

                self.sender.send(response)
            },

            MRequest::LeaveCrew => if let Some(ref mut spaceship) = self.spaceship {
                let token = self.sender.token();
                let index = spaceship.crew.iter()
                    .position(|member| member.token() == token)
                    .unwrap();

                spaceship.crew.remove(index);

                self.spaceship = None;

                let response = serde_json::to_string(&MResponse::LeaveCrew)
                    .map_err(io::Error::from)?;

                self.sender.send(response)
            } else {
                let response = serde_json::to_string(&MResponse::NotCrewed)
                    .map_err(io::Error::from)?;

                self.sender.send(response)
            }
        }
    }
}

fn handle(sender: Sender) -> impl Handler {
    println!("New connection! id={}", sender.connection_id());

    Server {
        sender,
        spaceship: None
    }
}

fn main() {
    println!("Hello, world!");

    ws::listen("0.0.0.0:8000", handle).unwrap();
}
