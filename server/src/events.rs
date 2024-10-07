use std::io;
use crossterm::event::Event;
use tokio::sync::mpsc::Receiver;


use crate::parser::parse_event;

pub struct EventStream {
    stdin: Receiver<Vec<u8>>,
}

impl EventStream {
    pub fn new(stdin: Receiver<Vec<u8>>) -> Self {
        EventStream { 
            stdin
        }
    }

    pub async fn next(&mut self) -> io::Result<Option<Event>> {
        let Some(msg) = self.stdin.recv().await else {
            return Ok(None);
        };
        parse_event(&msg, true)
    }
}
