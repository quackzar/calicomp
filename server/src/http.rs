use anyhow::{anyhow, Result};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, DefaultBodyLimit, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use calicomp::app::{events::update, App};
use crossterm::event::Event;
use futures::stream::SplitStream;
use futures_util::{SinkExt, StreamExt};
use ratatui::{prelude::*, Terminal};
use std::{io::Write, net::SocketAddr};
use tokio::{sync::mpsc::{self, Receiver}, task::JoinHandle};
use tokio::task;
use tokio::{net::TcpListener, sync::mpsc::Sender};
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{info, info_span, Instrument};

use crate::parser::{self, parse_event};

pub async fn start(addr: &SocketAddr) -> Result<()> {
    info!(?addr, "starting http server");

    let listener = TcpListener::bind(&addr).await?;

    let app = {
        let cors = CorsLayer::new()
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_origin(cors::Any);

        let trace = TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default());

        let limit = DefaultBodyLimit::max(512 * 1024);

        Router::new()
            .route("/", get(handle_connect))
            .layer(cors)
            .layer(trace)
            .layer(limit)
    };

    info!("ready");

    let app = app.into_make_service_with_connect_info::<SocketAddr>();

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_connect(
    socket: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let span = info_span!("websocket", %addr);
    let socket = socket.write_buffer_size(0);

    socket.on_upgrade(move |socket| {
        async move {
            info!("connection opened");

            match handle_connection(socket).await {
                Ok(()) => {
                    info!("connection closed");
                }

                Err(err) => {
                    info!("connection closed: {:?}", err);
                }
            }
        }
        .instrument(span)
    })
}

async fn handle_connection(socket: WebSocket) -> Result<()> {
    let mut instance = create_term(socket)?;
    tokio::task::spawn(async move {
        let res = instance.drive().await;
        tracing::info!("Instance stopped: {res:?}");
    });

    Ok(())
}

fn create_term(socket: WebSocket) -> Result<Instance> {
    let (mut stdout, stdin) = socket.split();

    let (stdout, task) = {
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(8);

        let task = task::spawn(
            async move {
                while let Some(msg) = rx.recv().await {
                    if stdout.send(Message::Binary(msg)).await.is_err() {
                        info!("couldn't push data into socket, killing stdout");
                        return;
                    }
                }
            }
            .in_current_span(),
        );
        (tx, task)
    };

    let writer = ProxyWriter {
        stdout,
        buffer: Vec::new(),
    };
    let backend = CrosstermBackend::new(writer);
    let term = Terminal::new(backend)?;

    let instance= Instance {
        pty: term,
        stdin,
        task,
        app: calicomp::app::App::new(),
    };

    Ok(instance)
}

type ProxyTerminal = Terminal<CrosstermBackend<ProxyWriter>>;


struct Instance {
    app: App,
    pty: ProxyTerminal,
    stdin: SplitStream<WebSocket>,
    task: JoinHandle<()>
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl Instance {
    pub async fn draw_state(&mut self) -> anyhow::Result<()> {
        self.pty.draw(|f| calicomp::ui::entry(f, &mut self.app))?;
        Ok(())
    }

    pub async fn drive(&mut self) -> anyhow::Result<()> {
        self.pty.clear()?;
        self.draw_state().await?;
        loop {
            if let Some(event) = self.next_event().await? {
                if self.step(event).await? {
                    break;
                }
            } 
        }

        Ok(())
    }

    pub async fn next_event(&mut self) -> anyhow::Result<Option<Event>> {
        let msg = self.stdin.next().await.ok_or_else(||anyhow!("stdin closed"))?;

        let bytes = match msg {
            Ok(Message::Text(msg)) => {
                msg.into_bytes()
            }

            Ok(Message::Binary(msg)) => {
                msg
            }

            Ok(_) => {
                return Err(anyhow!("Can't handle this"));
            }

            Err(err) => {
                return Err(err.into())
            }
        };

        if let Some(size) = bytes.strip_prefix(&[0x04]) {
            let cols = size.first().copied().unwrap_or(0) as u16;
            let rows = size.last().copied().unwrap_or(0) as u16;
            tracing::debug!("resized to {rows} x {cols}");

            self.pty.resize(Rect { x: 0, y: 0, width: cols, height: rows })?;
            self.draw_state().await?;
            Ok(None)
        } else {
            Ok(parse_event(&bytes, false)?)
        }


    }

    pub async fn step(&mut self, event: Event) -> anyhow::Result<bool> {
        update(&mut self.app, event).await.unwrap();
        self.pty.draw(|frame| calicomp::ui::entry(frame, &mut self.app))?;
        if self.app.should_quit {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}


pub struct EventHandler {
    channel: Receiver<Event>,
}


#[derive(Debug)]
struct ProxyWriter {
    buffer: Vec<u8>,
    stdout: Sender<Vec<u8>>,
}

impl Write for ProxyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let msg = std::mem::take(&mut self.buffer);
        // This is probably the right way, if the buffer gets full
        // or channel closed it will be an error, but that seems correct.
        // We will just need to be able to recover from such errors.
        self.stdout
            .try_send(msg)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))
    }
}
