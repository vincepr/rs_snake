# Snake Game in Rust (with wasm)
Implementing Webassembly Version of a Classic Snake game. Trying to use only neccessary dependencies on this one.

My conclusion after this project, Really did not like this whole frontend with raw rust-wasm thing one bit. Never do this again!

## Live Demo
- https://vincepr.github.io/rs_snake/

## notes
- check the random-module to see how to use native JS functions (like `Math.random() console.log()`)
- building with `wasm-pack build --out-dir ./build/pkg --target web` then hosting the build folder with a server.