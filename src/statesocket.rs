use std::thread;
use std::sync::{RwLock};
use grid::Point;
use entity::Entity;
use ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};

pub struct Server {
        pub out: Sender,
}
impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.out.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}

pub fn send_message(json: String) {
    let client = thread::spawn(move || {
        connect("ws://127.0.0.1:3012", |out| {
            let message = json.clone();
            out.send(message).unwrap();

            move |msg| {
                println!("{}", msg);
                out.close(CloseCode::Normal)
            }
        })
    });
}

