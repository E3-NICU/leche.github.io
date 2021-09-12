## About
**Leche** is a tool to help medical staff warm up baby milk to a target temperature.

## Extending
This project is written in Rust/WebAssembly.
To change the website you must recompile the code to a new wasm file.

Learning how to install Rust can be found [here](https://www.rust-lang.org/tools/install).

We also need [wasm-pack](https://github.com/rustwasm/wasm-pack) to compile Rust into WebAssembly.

To install this use the command ``cargo install wasm-pack``.

Now run ``build --target web --out-name wasm --out-dir ..`` in the code folder to generate the program.

The code for the website can be found in the **code** folder. 
This contains a the src folder, some frontend libraries and Cargo.toml .

## Authors
This project was made by Thomas Dooms in collaboration with Charlie Beirnaert and Dr. Ludo Mahieu.
