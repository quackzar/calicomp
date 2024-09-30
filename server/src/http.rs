use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, DefaultBodyLimit, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures_util::{SinkExt, StreamExt};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{io::Write, net::SocketAddr};
use tokio::sync::mpsc;
use tokio::task;
use tokio::{net::TcpListener, sync::mpsc::Sender};
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{info, info_span, Instrument};

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
    let mut term = create_term(socket)?;

    // TODO: Use a async-version of the app
    tokio::task::spawn_blocking(move || {
        let mut app = calicomp::app::App::new();
        calicomp::app::events::run_app(&mut term, &mut app)
    })
    .await??;

    Ok(())
}

fn create_term(socket: WebSocket) -> Result<ProxyTerminal> {
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

                        Err(_) => {
                            info!("couldn't pull data from socket, killing stdin");

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
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(1);

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


    tokio::spawn(async move {
        let mut stdin = stdin;
        loop {
            stdin.recv().await;
        }
    });

    Ok(term)
}

type ProxyTerminal = Terminal<CrosstermBackend<ProxyWriter>>;

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
        let _ = self.stdout.blocking_send(msg);
        Ok(())
    }
}
