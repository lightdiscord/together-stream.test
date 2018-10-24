extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate ws;
extern crate uuid;
extern crate messages;

use ws::util::Token;
use ws::{ Sender, Handler };
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

pub struct Peer {
    sender: Sender,

    spaceship: Option<Spaceship>,

    state: Arc<Mutex<Shared>>,
}

#[derive(Debug)]
struct Shared {
    peers: HashMap<Token, Sender>,
    spaceships: HashMap<Uuid, Spaceship>
}

#[derive(Debug)]
struct Spaceship {
    captain: Sender,
    crew: Vec<Sender>,
    id: Uuid,
}

impl Spaceship {
    fn new(captain: Sender) -> Self {
        Spaceship {
            crew: vec![captain.clone()],
            captain,
            id: Uuid::new_v4()
        }
    }
}

impl Shared {
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
            spaceships: HashMap::new()
        }
    }
}

impl Peer {
    fn new(sender: Sender, state: Arc<Mutex<Shared>>) -> Self {
        let token = sender.token();

        state.lock().unwrap().peers.insert(token, sender.clone());

        Peer {
            sender,
            spaceship: None,
            state
        }
    }

    pub fn token(&self) -> Token {
        self.sender.token()
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        self.state.lock().unwrap().peers.remove(&self.token());
    }
}

impl Handler for Peer {
    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        for (key, value) in self.state.lock().unwrap().peers.iter() {
            value.send(message.clone())?;
        }
        self.sender.send(message)
    }
}

fn main() {
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("0.0.0.0:{}", port);

    let state = Arc::new(Mutex::new(Shared::new()));

    let handler = |sender: Sender| {
        Peer::new(sender, state.clone())
    };

    println!("Listening on {}", address);

    let listener = ws::listen(address, handler);
    let _ = listener.unwrap();
}
