use std::collections::HashMap;
use std::convert::Infallible;
use std::path::Path;
use std::sync::Arc;

use futures_util::StreamExt;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::ws::Message;
use warp::{Filter, Rejection, Reply};

type Clients = Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let static_files = warp::path("pkg")
        .and(warp::fs::dir("./pkg"))
        .or(warp::path("contents").and(warp::fs::dir("./contents")))
        .or(warp::fs::file("./index.html"));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    let routes = static_files.or(ws_route);

    println!("🚀 Starting docs dev server at http://127.0.0.1:8080");
    println!("♻️ Watching for changes...");

    let watcher_clients = clients.clone();
    tokio::spawn(async move {
        watch_files(watcher_clients).await;
    });

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, clients)))
}

async fn handle_socket(socket: warp::ws::WebSocket, clients: Clients) {
    let (tx, mut rx) = socket.split();
    let (client_tx, client_rx) = tokio::sync::mpsc::unbounded_channel();

    let client_id = Uuid::new_v4().to_string();
    clients.write().await.insert(client_id.clone(), client_tx);

    tokio::spawn(async move {
        tokio::io::copy(client_rx, tx).await.unwrap();
    });

    while let Some(_) = rx.next().await {
        // Handle incoming messages if needed
    }

    clients.write().await.remove(&client_id);
}

async fn watch_files(clients: Clients) {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(Path::new("src"), RecursiveMode::Recursive).unwrap();
    watcher
        .watch(Path::new("contents"), RecursiveMode::Recursive)
        .unwrap();
    watcher.watch(Path::new("index.html"), RecursiveMode::NonRecursive).unwrap();

    for res in rx {
        match res {
            Ok(_) => {
                println!("♻️ Change detected. Rebuilding...");
                rebuild().await;
                broadcast_reload(&clients).await;
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

async fn rebuild() {
    let output = std::process::Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--out-dir",
            "pkg",
            "--out-name",
            "bahlilui_docs",
        ])
        .output()
        .expect("Failed to run wasm-pack");

    if output.status.success() {
        println!("✅ Build successful");
    } else {
        println!("❌ Build failed");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

async fn broadcast_reload(clients: &Clients) {
    let clients = clients.read().await;
    for (_, sender) in clients.iter() {
        let _ = sender.send(Message::text("reload"));
    }
}
