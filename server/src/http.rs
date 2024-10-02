use anyhow::{anyhow, Result};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, DefaultBodyLimit, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use calicomp::app::{events, CurrentMode, CurrentScreen};
use crossterm::event::Event;
use futures_util::{SinkExt, StreamExt};
use ratatui::{prelude::*, Terminal};
use std::{io::Write, net::SocketAddr, time::Duration};
use tokio::sync::mpsc::{self, Receiver};
use tokio::task;
use tokio::{net::TcpListener, sync::mpsc::Sender};
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{info, info_span, Instrument};

use termwiz::input::{InputEvent, InputParser};

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
    let (mut term, mut handler) = create_term(socket)?;
    tokio::task::spawn(async move {
        let mut app = calicomp::app::App::new();
        loop {
            tokio::time::sleep(Duration::from_millis(200)).await;
            term.draw(|f| calicomp::ui::entry(f, &mut app)).unwrap();
            let Ok(event) = handler.next().await else {break};
            tracing::debug!("event: {event:?}");
            match event {
                InputEvent::Key(key_event) => {
                    match key_event.key {
                        termwiz::input::KeyCode::Char('e') => {
                            app.current_mode = CurrentMode::Editing;
                        },
                        termwiz::input::KeyCode::Char('k') => {
                            app.list_state.select_next();
                        },
                        _ => {},
                    }
                },
                InputEvent::Resized { cols, rows } => {
                    tracing::debug!("resized to {cols} x {rows}");
                },
                _ => {},
            }

        }
    })
    .await?;

    Ok(())
}

fn create_term(socket: WebSocket) -> Result<(ProxyTerminal, EventHandler)> {
    let (mut stdout, mut stdin) = socket.split();

    let stdin = {
        let (tx, rx) = mpsc::channel(1);
        task::spawn(
            async move {
                while let Some(msg) = stdin.next().await {
                    match msg {
                        Ok(Message::Text(msg)) => {
                            if tx.send(msg.into_bytes()).await.is_err() {
                                break;
                            }
                        }

                        Ok(Message::Binary(msg)) => {
                            if tx.send(msg).await.is_err() {
                                break;
                            }
                        }

                        Ok(_) => {
                            //
                        }

                        Err(err) => {
                            info!("couldn't pull data from socket, killing stdin: {err}");

                            return;
                        }
                    }
                }
            }
            .in_current_span(),
        );

        rx
    };

    let stdout = {
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(8);

        task::spawn(
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

        tx
    };

    let writer = ProxyWriter {
        stdout,
        buffer: Vec::new(),
    };
    let backend = CrosstermBackend::new(writer);
    let term = Terminal::new(backend)?;

    let handler = EventHandler::new(stdin);

    Ok((term, handler))
}

type ProxyTerminal = Terminal<CrosstermBackend<ProxyWriter>>;

pub struct EventHandler {
    channel: Receiver<InputEvent>,
}

impl EventHandler {
    pub fn new(stdin: Receiver<Vec<u8>>) -> Self {
        let (sx, rx) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            let mut input_parser = InputParser::new();
            let mut stdin = stdin;
            loop {
                let msg = stdin.recv().await.expect("Stdin closed");
                input_parser.parse(&msg, |e| {
                    let _ =sx.try_send(e);
                }, true);
            };
        });

        EventHandler { channel: rx }

    }
    pub async fn next(&mut self) -> Result<InputEvent> {
        self.channel.recv().await.ok_or_else(|| {
            anyhow!("Stream closed")
        })
    }
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
