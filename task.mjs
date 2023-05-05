argv.debug = argv.debug == null ? 1 : +argv.debug;
console.log('debug: ', argv.debug)
if (argv.debug == 1) {
    await $`cargo build --target wasm32-unknown-unknown`
    await $`wasm-bindgen target/wasm32-unknown-unknown/debug/gluttonous-snake.wasm --out-dir wasm --web --out-name snake`
} else {
    await $`cargo build --target wasm32-unknown-unknown --release`
    await $`wasm-bindgen target/wasm32-unknown-unknown/release/gluttonous-snake.wasm --out-dir wasm --web --out-name snake`
}

await $`http-server .`