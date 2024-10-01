use eyre::{OptionExt, Result};
use std::time::Duration;

use crossterm::event::Event;

pub struct EventHandler {
    rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let tick_rate = Duration::from_millis(50);
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(tick_rate).await;
                if let Ok(true) = crossterm::event::poll(Duration::from_secs(0)) {
                    let event = crossterm::event::read().unwrap();
                    let _ = tx.send(event);
                }
            }
        });

        EventHandler { rx, task }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx.recv().await.ok_or_eyre("Event stream closed")
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Tui {}
