
// adapted from
// https://github.com/Eugeny/russh/blob/main/russh/examples/ratatui_app.rs


use std::sync::Arc;
use std::collections::HashMap;

use async_trait::async_trait;
use crossterm::event::Event;
use ed25519_dalek::{pkcs8::{spki::der::pem::LineEnding::LF, DecodePrivateKey, EncodePrivateKey}, SigningKey};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use russh::keys::key::PublicKey;
use russh::server::*;
use russh::{Channel, ChannelId};
use russh_keys::key::KeyPair;
use tokio::sync::Mutex;

type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

#[derive(Clone)]
struct TerminalHandle {
    handle: Handle,
    // The sink collects the data which is finally flushed to the handle.
    sink: Vec<u8>,
    channel_id: ChannelId,
}

struct Instance {
    terminal: SshTerminal,
    app: App,
}

impl Instance {
    pub async fn step(&mut self, event: Event) -> anyhow::Result<bool> {
        update(&mut self.app, event).await.unwrap();
        self.terminal.draw(|frame| ui::entry(frame, &mut self.app))?;
        if self.app.should_quit {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn draw_state(&mut self) -> anyhow::Result<()> {
        self.terminal.draw(|frame| ui::entry(frame, &mut self.app))?;
        Ok(())
    }
}

use calicomp::{app::{events::{self, update}, App}, tui::EventHandler, ui};

use crate::parser;

// The crossterm backend writes to the terminal handle.
impl std::io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let handle = self.handle.clone();
        let channel_id = self.channel_id;
        let data = self.sink.clone().into();
        futures::executor::block_on(async move {
            let result = handle.data(channel_id, data).await;
            if result.is_err() {
                eprintln!("Failed to send data: {:?}", result);
            }
        });

        self.sink.clear();
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppServer {
    clients: Arc<Mutex<HashMap<usize, Instance>>>,
    id: usize,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        let key = if let Ok(keyfile) = tokio::fs::read_to_string("./keypair").await {
            tracing::info!("Loaded private key");
            let key = SigningKey::from_pkcs8_pem(&keyfile)?; 
            KeyPair::Ed25519(key)
        } else {
            tracing::info!("No keypair found at './keypair', generating new");
            let key = KeyPair::generate_ed25519().unwrap();
            let KeyPair::Ed25519(k) = &key else { panic!() };
            let res = SigningKey::to_pkcs8_pem(k, LF)?;
            tokio::fs::write("keypair", res).await?;
            key
        };

        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            keys: vec![key],
            ..Default::default()
        };

        self.run_on_address(Arc::new(config), ("0.0.0.0", 2222))
            .await?;
        Ok(())
    }
}

impl Default for AppServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Server for AppServer {
    type Handler = Self;
    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

#[async_trait]
impl Handler for AppServer {
    type Error = anyhow::Error;

    #[tracing::instrument(skip_all)]
    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        {
            let mut clients = self.clients.lock().await;
            let terminal_handle = TerminalHandle {
                handle: session.handle(),
                sink: Vec::new(),
                channel_id: channel.id(),
            };

            let backend = CrosstermBackend::new(terminal_handle.clone());
            let mut terminal = Terminal::new(backend)?;
            let app = App::new();

            tracing::info!("Got new terminal");

            terminal.clear()?;

            let mut instance = Instance { terminal, app };
            instance.draw_state().await?;
            clients.insert(self.id, instance);
        }

        Ok(true)
    }

    async fn auth_publickey(&mut self, _: &str, _: &PublicKey) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut lock = self.clients.lock().await;
        let event = parser::parse_event(data, false)?;
        if let Some(event) = event {
            let instance = lock.get_mut(&self.id).unwrap();
            let should_quit = instance.step(event).await?;
            if should_quit {
                lock.remove(&self.id);
            }
        }
        Ok(())
    }

    /// The client's window size has changed.
    async fn window_change_request(
        &mut self,
        _: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &mut Session,
    ) -> Result<(), Self::Error> {
        tracing::debug!("resized to {row_height} x {col_width}");
        {
            let mut clients = self.clients.lock().await;
            let instance = clients.get_mut(&self.id).unwrap();

            let width = col_width.min(255);
            let height = row_height.min(255);
            let rect = Rect {
                x: 0,
                y: 0,
                width: width as u16,
                height: height as u16,
            };
            instance.terminal.resize(rect)?;
            instance.draw_state().await?;
        }

        Ok(())
    }
}

