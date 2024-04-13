//import init, { start } from "./pkg/rust_webgl_renderer.js";
import init, { start } from "./pkg/vaporization.js";

async function run() {
    await init();
    start();
}

run();
