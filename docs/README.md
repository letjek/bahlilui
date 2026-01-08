# BahlilUI Documentation Site

This is the official documentation site for BahlilUI, built using BahlilUI itself! 🌶️

## Building the Docs

The documentation site is a BahlilUI application that showcases the framework's capabilities.

### Prerequisites

- Rust (via `rustup`)
- `wasm-pack`

### Build Steps

1. **Install wasm-pack** (if not already installed):
   ```bash
   cargo install wasm-pack
   ```

2. **Build the WebAssembly bundle**:
   ```bash
   cd docs
   npm run build
   # or directly:
   wasm-pack build --target web --out-dir pkg --out-name bahlilui_docs
   ```

3. **Serve locally** (optional):
   ```bash
   npm run serve
   # or:
   python3 -m http.server 8000
   ```
   Then open `http://localhost:8000`

### Deployment

The built `pkg/` directory contains the WebAssembly files and should be committed to the repository for deployment.

For Vercel deployment:
1. Push the code (including the `pkg/` directory) to your repository
2. Connect the repository to Vercel
3. Vercel will automatically serve the static files

## Project Structure

```
docs/
├── contents/
│   ├── view.bui      # Main template with placeholders
│   └── style.css     # Styles for the docs site
├── src/
│   └── lib.rs        # Rust logic that powers the docs
├── pkg/              # Built WebAssembly files (generated)
├── index.html        # Entry point
├── package.json      # Build scripts
└── README.md         # This file
```

## How It Works

This documentation site demonstrates BahlilUI in action:

- **Template**: `contents/view.bui` contains HTML with `{{placeholders}}`
- **Styles**: `contents/style.css` provides the visual design
- **Logic**: `src/lib.rs` defines the app data and rendering logic
- **Build**: Everything gets compiled into a single WebAssembly bundle

The site is fully interactive and showcases all of BahlilUI's features while documenting how to use the framework.