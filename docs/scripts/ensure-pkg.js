const fs = require("fs");
const path = require("path");

const { spawnSync } = require("child_process");

const pkgFile = path.join(__dirname, "..", "pkg", "bahlilui_docs.js");

if (fs.existsSync(pkgFile)) {
  console.log("Found pkg artifacts. Skipping wasm-pack build on CI.");
  process.exit(0);
}

const wasmPackCheck = spawnSync("wasm-pack", ["--version"], {
  stdio: "ignore",
});

if (wasmPackCheck.status !== 0) {
  console.error(
    "Missing docs/pkg/bahlilui_docs.js and wasm-pack is not available. Run wasm-pack build locally and commit pkg/ for Vercel."
  );
  process.exit(1);
}

const build = spawnSync(
  "wasm-pack",
  ["build", "--target", "web", "--out-dir", "pkg", "--out-name", "bahlilui_docs"],
  { stdio: "inherit", cwd: path.join(__dirname, "..") }
);

if (build.status !== 0) {
  console.error("wasm-pack build failed.");
  process.exit(build.status || 1);
}
