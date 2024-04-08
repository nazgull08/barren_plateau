import init, { start } from "./pkg/rust_webgl_renderer.js";

async function run() {
    await init();
    start();
}

run();
