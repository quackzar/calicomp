
// adapted from
// https://github.com/Eugeny/russh/blob/main/russh/examples/ratatui_app.rs


use std::sync::Arc;
use std::collections::HashMap;

use async_trait::async_trait;
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

use calicomp::{app::App, ui};

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
    clients: Arc<Mutex<HashMap<usize, (SshTerminal, App)>>>,
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
        let clients = self.clients.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;
                for (_, (terminal, app)) in clients.lock().await.iter_mut() {
                    // We can't reuse events here, as it would be blocking.
                    terminal.draw(|f| ui::entry(f, app)).unwrap();

                }
            }
        });

        let key = if let Ok(key) = tokio::fs::read_to_string("./keypair").await {
            tracing::info!("Loaded private key");
            let key = SigningKey::from_pkcs8_pem(&key)?; 
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
            let terminal = Terminal::new(backend)?;
            let app = App::new();

            tracing::info!("Got new terminal");
            clients.insert(self.id, (terminal, app));
        }

        Ok(true)
    }

    async fn auth_publickey(&mut self, _: &str, _: &PublicKey) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        
        match data {
            // Pressing 'q' closes the connection.
            b"q" => {
                self.clients.lock().await.remove(&self.id);
                session.close(channel);
            }
            // Pressing 'c' resets the counter for the app.
            // Only the client with the id sees the counter reset.
            b"c" => {
                let mut clients = self.clients.lock().await;
                let (_, _app) = clients.get_mut(&self.id).unwrap();
            }
            _ => {}
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
        {
            let mut clients = self.clients.lock().await;
            let (terminal, _) = clients.get_mut(&self.id).unwrap();
            let rect = Rect {
                x: 0,
                y: 0,
                width: col_width as u16,
                height: row_height as u16,
            };
            terminal.resize(rect)?;
        }

        Ok(())
    }
}

