#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate ws;
extern crate uuid;

pub mod messages;
pub mod controllers;

use ws::{ Sender, Handler, Request, Response };
use self::messages::Incoming;
use self::controllers::{ Server };

impl Handler for Server {
    fn on_request(&mut self, request: &Request) -> ws::Result<(Response)> {
        match request.resource() {
            "/ws" => Response::from_request(request),
            "/" => Ok(Response::new(200, "OK", b"You should go to /ws".to_vec())),
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        <Incoming as controllers::Handler>::on_message(self, &message)
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
