# Mandelbrot WASM
This repository contains a basic implementation of a Mandelbrot Set viewer which runs on WASM. It uses NPM to serve the web page containing the WASM.

The goal here was to get basic familiarity with WASM as a Rust target.

# Building and running
This readme was written well after the code was last changed. I am going based on memory for this section - some parts may be incorrect!

Follow the instructions here to install the necessary tools. In particular, wasm-pack and npm.
https://rustwasm.github.io/docs/book/game-of-life/setup.html

To build the project:
`wasm-pack build`

Get NPM installed:
`npm install`

Then run the web server with NPM:
`npm run start`

Now you can access the web page at http://localhost:8080/
