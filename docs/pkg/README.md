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
   Note: if you add DOM APIs like `query_selector_all`, make sure the relevant `web-sys` features (e.g. `NodeList`) are enabled in `docs/Cargo.toml`.

3. **Serve locally** (optional):
   ```bash
   npm run dev
   # or if you only want static hosting without rebuilds:
   npm run serve
   ```
   Then open `http://localhost:8000`

### Deployment

The built `pkg/` directory contains the WebAssembly files and should be committed to the repository for deployment.

For Vercel deployment:
1. Run `wasm-pack build --target web --out-dir pkg --out-name bahlilui_docs`
2. Commit the generated `pkg/` directory
3. Push the code (including `pkg/`) to your repository
4. Connect the repository to Vercel
5. Vercel will run `npm run build`, which checks for `pkg/` and skips rebuilding

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
