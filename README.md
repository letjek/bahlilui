# 🌶️ BahlilUI

**The "No‑Nonsense" Rust → WebAssembly UI framework.**  
Write UI logic in **Rust**, keep styling in **CSS**, and author templates in **`.bui`** (HTML with superpowers). Compile everything into a **single `.wasm`** bundle.

![BahlilUI](https://img.shields.io/badge/BahlilUI-Rust%20WASM-dea584?style=for-the-badge&logo=rust&logoColor=black)

📚 **[Read the Docs](https://bahlilui.vercel.app)** (Built with BahlilUI!)

---

## ✨ What is BahlilUI?

BahlilUI is a tiny, compile-time friendly UI approach:

- **Templates**: `.bui` files (HTML + `{{placeholders}}`)
- **Styles**: plain `.css`
- **Logic**: Rust structs + simple replacement renderer
- **Output**: a WASM app that injects CSS + renders HTML into the DOM

> Think: “keep it simple, ship it fast” — no virtual DOM, no mega build system, just Rust + WASM.

---

## ✅ Features

- **⚡ Fast startup**: minimal runtime overhead
- **🔒 Type‑safe state**: app data is Rust structs
- **📦 Single bundle**: `.bui` + `.css` embedded into `.wasm` via `include_str!`
- **♻️ Hot reload dev server**: watches `.rs`, `.bui`, `.css` and reloads browser
- **🧩 Minimal mental model**: string template rendering + DOM injection

---

## 🧰 Requirements

- Rust (via `rustup`)
- `wasm-pack`
- A modern browser (for WebAssembly)

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

### Install BahlilUI CLI

```bash
cargo install --path .
```

---

## 🌐 Documentation Site

The [BahlilUI documentation site](https://bahlilui.vercel.app) is itself built using BahlilUI! This demonstrates the framework's capabilities in a real-world application.

To build the docs locally:
```bash
cd docs
wasm-pack build --target web --out-dir pkg --out-name bahlilui_docs
# Then serve with any static server
```

---

## 🛠️ CLI Usage

BahlilUI comes with a CLI tool to help you create and manage projects.

### Create a new project

```bash
bui new my_app
cd my_app
```

### Run development server

```bash
bui dev
```

This starts a hot-reload dev server at `http://127.0.0.1:8080`.

### Build for production

```bash
bui build
```

---

## 🚀 Quick Start

### 1) Install BahlilUI CLI

```bash
cargo install --path .
```

### 2) Create a new project

```bash
bui new bahlilui_app
cd bahlilui_app
```

### 3) Run the development server

```bash
bui dev
```

Then open `http://127.0.0.1:8080` in your browser.

The CLI will generate all necessary files and start the hot-reload server automatically.

---

## 📁 Project Structure

A typical BahlilUI project looks like this:

```text
.
├── contents/
│   ├── view.bui
│   └── style.css
├── src/
│   ├── lib.rs
│   └── bin/
│       └── dev_server.rs
├── index.html
└── Cargo.toml
```

### contents/view.bui

```html
<div class="card">
  <h1>{{title}}</h1>
  <p>Welcome, <strong>{{username}}</strong>!</p>
  <p>This is rendered via BahlilUI.</p>
</div>
```

### contents/style.css

```css
body {
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

h1 { color: #dea584; }
```

### src/lib.rs

```rust
use wasm_bindgen::prelude::*;
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
}
```

### index.html

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>BahlilUI App</title>
  </head>
  <body>
    <script type="module">
      import init from "./pkg/bahlilui_app.js";
      init();

      const socket = new WebSocket("ws://" + window.location.host + "/ws");
      socket.onmessage = (event) => {
        if (event.data === "reload") {
          console.log("♻️ Change detected. Reloading...");
          window.location.reload();
        }
      };
    </script>
  </body>
</html>
```

---

## 🧪 Development Workflow

### Build WASM bundle

```bash
wasm-pack build --target web
```

This creates the `pkg/` folder used by `index.html`.
If you add DOM APIs like `query_selector_all`, ensure the corresponding `web-sys` features
(e.g. `NodeList`) are enabled in `Cargo.toml`.

### Run the dev server (hot reload)

```bash
cargo run --bin dev_server
```

Then open:

- `http://127.0.0.1:8080`

---

## ♻️ Hot Reload Dev Server (Concept)

A minimal dev server can:

- Serve static files (`index.html`, `pkg/`, `contents/`)
- Open a WebSocket endpoint (`/ws`)
- Watch your project for changes:
  - `.rs`, `.bui`, `.css`
- On change:
  - rebuild via `wasm-pack build --target web`
  - broadcast `"reload"` to connected browsers

---

## 🧩 The `.bui` Template Format

`.bui` is **HTML with Superpowers**:

```html
<h1>{{title}}</h1>
<p>Hello, {{username}}!</p>
```

At runtime (or at "render time"), placeholders are replaced by strings from your Rust state.

### Notes & Conventions

- Placeholders are wrapped with `{{ }}`.
- Replacement is a simple string substitution.
- For production use, consider:
  - escaping HTML
  - validating placeholders
  - richer syntax (conditionals, loops)

---

## 📦 Build & Deploy

### 1) Build

```bash
wasm-pack build --target web --release
```

### 2) Serve

Any static file server works (because output is browser WASM + JS glue + HTML). Examples:

- `python -m http.server`
- `nginx`
- `caddy`

---

## 🗺️ Roadmap (Ideas)

If you want to evolve BahlilUI into a real framework:

- **Template engine**: loops/conditionals + safe escaping
- **Event binding**: `on:click="{{handler}}"` mapping to Rust functions
- **Component system**: nested templates + scoped CSS
- **State updates**: partial re-render / targeted DOM updates
- **CLI**: `bui new`, `bui dev`, `bui build`

---

## 🤝 Contributing

PRs welcome:

1. Fork the repo
2. Create a feature branch
3. Add tests/examples
4. Submit PR

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙌 Credits

Built with ❤️ in Rust + WebAssembly.

> “Simple UI, spicy name.” 🌶️
