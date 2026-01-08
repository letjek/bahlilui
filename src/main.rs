use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "bui")]
#[command(about = "BahlilUI CLI - The No-Nonsense Rust → WebAssembly UI framework generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new BahlilUI project
    New {
        /// Name of the project
        name: String,
    },
    /// Run the development server with hot reload
    Dev,
    /// Build the project for production
    Build,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            create_new_project(&name)?;
        }
        Commands::Dev => {
            run_dev_server().await?;
        }
        Commands::Build => {
            build_project()?;
        }
    }

    Ok(())
}

fn create_new_project(name: &str) -> anyhow::Result<()> {
    let project_dir = Path::new(name);

    if project_dir.exists() {
        anyhow::bail!("Directory {} already exists", name);
    }

    fs::create_dir_all(project_dir)?;

    // Create subdirectories
    fs::create_dir_all(project_dir.join("contents"))?;
    fs::create_dir_all(project_dir.join("src/bin"))?;

    // Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[[bin]]
name = "dev_server"
path = "src/bin/dev_server.rs"

[dependencies]
wasm-bindgen = "0.2"
web-sys = {{ version = "0.3", features = ["Document", "Element", "HtmlElement", "Node", "NodeList", "Window", "HtmlStyleElement", "HtmlHeadElement"] }}
warp = "0.3"
tokio = {{ version = "1", features = ["full"] }}
notify = "6.1"
futures-util = "0.3"
"#, name);
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    // contents/view.bui
    let view_bui = r#"<div class="card">
  <h1>{{title}}</h1>
  <p>Welcome, <strong>{{username}}</strong>!</p>
  <p>This is rendered via BahlilUI.</p>
</div>"#;
    fs::write(project_dir.join("contents/view.bui"), view_bui)?;

    // contents/style.css
    let style_css = r#"body {
  font-family: sans-serif;
  background: #222;
  color: #fff;
  display: flex;
  justify-content: center;
  margin-top: 50px;
}

.card {
  background: #333;
  padding: 2rem;
  border-radius: 10px;
  border: 1px solid #dea584;
  text-align: center;
}

h1 { color: #dea584; }"#;
    fs::write(project_dir.join("contents/style.css"), style_css)?;

    // src/lib.rs
    let lib_rs = r#"use wasm_bindgen::prelude::*;
use web_sys::window;

const HTML_TEMPLATE: &str = include_str!("../contents/view.bui");
const CSS_STYLES: &str = include_str!("../contents/style.css");

struct AppData {
    title: String,
    username: String,
}

fn render(template: &str, data: &AppData) -> String {
    let mut output = template.to_string();
    output = output.replace("{{title}}", &data.title);
    output = output.replace("{{username}}", &data.username);
    output
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let document = window().expect("no window").document().expect("no document");
    let body = document.body().expect("no body");
    let head = document.head().expect("no head");

    // inject CSS
    let style = document.create_element("style")?;
    style.set_inner_html(CSS_STYLES);
    head.append_child(&style)?;

    // app data (state)
    let data = AppData {
        title: "BahlilUI".to_string(),
        username: "Developer".to_string(),
    };

    // render HTML
    let container = document.create_element("div")?;
    container.set_inner_html(&render(HTML_TEMPLATE, &data));
    body.append_child(&container)?;

    Ok(())
}"#;
    fs::write(project_dir.join("src/lib.rs"), lib_rs)?;

    // src/bin/dev_server.rs
    let dev_server_rs = r#"use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::ws::Message;
use warp::{Filter, Rejection, Reply};
use futures_util::StreamExt;
use uuid::Uuid;

type Clients = Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let static_files = warp::path("pkg").and(warp::fs::dir("./pkg"))
        .or(warp::path("contents").and(warp::fs::dir("./contents")))
        .or(warp::fs::file("./index.html"));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    let routes = static_files.or(ws_route);

    println!("🚀 Starting dev server at http://127.0.0.1:8080");
    println!("♻️ Watching for changes...");

    // Start file watcher
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
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(Path::new("src"), RecursiveMode::Recursive).unwrap();
    watcher.watch(Path::new("contents"), RecursiveMode::Recursive).unwrap();

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
        .args(&["build", "--target", "web"])
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
}"#;
    fs::write(project_dir.join("src/bin/dev_server.rs"), dev_server_rs)?;

    // index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{} - BahlilUI App</title>
  </head>
  <body>
    <script type="module">
      import init from "./pkg/{}.js";
      init();

      const socket = new WebSocket("ws://" + window.location.host + "/ws");
      socket.onmessage = (event) => {{
        if (event.data === "reload") {{
          console.log("♻️ Change detected. Reloading...");
          window.location.reload();
        }}
      }};
    </script>
  </body>
</html>"#, name, name);
    fs::write(project_dir.join("index.html"), index_html)?;

    println!("✅ Created new BahlilUI project: {}", name);
    println!("   cd {}", name);
    println!("   bui dev");

    Ok(())
}

async fn run_dev_server() -> anyhow::Result<()> {
    // Check if we're in a BahlilUI project
    if !Path::new("Cargo.toml").exists() || !Path::new("contents").exists() {
        anyhow::bail!("Not in a BahlilUI project directory");
    }

    // First, build once
    println!("🔨 Initial build...");
    let status = Command::new("wasm-pack")
        .args(["build", "--target", "web"])
        .status()?;
    if !status.success() {
        anyhow::bail!("Initial build failed");
    }

    // Then run the dev server
    println!("🚀 Starting dev server...");
    Command::new("cargo")
        .args(["run", "--bin", "dev_server"])
        .status()?;

    Ok(())
}

fn build_project() -> anyhow::Result<()> {
    println!("🔨 Building for production...");
    let status = Command::new("wasm-pack")
        .args(["build", "--target", "web", "--release"])
        .status()?;
    if status.success() {
        println!("✅ Build successful");
    } else {
        anyhow::bail!("Build failed");
    }
    Ok(())
}
