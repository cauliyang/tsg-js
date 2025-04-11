::: {align="center"}
<h1><code>tsg-js</code></h1>

<strong>JavaScript bindings for the tsg Rust library using WebAssembly</strong>

<p><a href="https://github.com/yourusername/tsg-js"><img src="https://img.shields.io/github/workflow/status/yourusername/tsg-js/CI?style=flat-square" alt="Build Status"/></a></p>
:::

## About

This project provides JavaScript bindings for the `tsg` Rust library, compiled to WebAssembly to enable usage in web applications and Node.js environments.

## 🚴 Usage

### 🛠️ Building the Project

```
wasm-pack build
```

### 🔬 Running Tests

```
wasm-pack test --headless --firefox
```

### 🎁 Publishing to NPM

```bash
wasm-pack publish
```

### 📦 Using in Your Project

After installation:

```js
import * as tsg from 'tsg-core-js';

async function load_tsg() {
	await tsg.default();
	console.log('tsg loaded');
}
// Use the tsg library functions
// Example usage goes here
```

## 🔋 Features

- Access the full functionality of the tsg Rust library from JavaScript
- High-performance computations using WebAssembly
- Compatible with both browser and Node.js environments

## 🔧 Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/en/download/)

### Setup

```bash
git clone https://github.com/yourusername/tsg-js.git
cd tsg-js
wasm-pack build
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
