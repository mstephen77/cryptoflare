# Cryptoflare

Cryptoflare is a lightweight library designed to enhance Cloudflare Workers with robust cryptographic functionality. By utilizing Rust and WebAssembly (WASM), Cryptoflare provides high-performance and secure cryptographic operations within the Cloudflare Workers environment.

## Installation

To use cryptoflare, clone this repository and deploy it to your Cloudflare account. Make sure you have `wrangler` and Rust's toolchain installed.

```sh
git clone git@github.com:mstephen77/cryptoflare.git
cd ./cryptoflare
npm install
npm run deploy
```

## Usage

Use cryptoflare by [binding](https://developers.cloudflare.com/workers/runtime-apis/bindings/service-bindings) the cryptoflare worker to another worker, then call:

```js
const result = await (await env.CRYPTOFLARE.fetch(ENDPOINT, {
  method: 'POST',
  body: JSON.stringify(DATA),
})).json();
```

### API Specification

#### Hash

- Endpoints:
  - `/argon2/hash`, with available `options`:
    ```JSON
    {
      "time_cost": 2,
      "memory_cost": 19456, // = 19 * 1024
      "parallelism": 1
    }
    ```
  - `/bcrypt/hash`, with available `options`:
    ```JSON
    { "work_factor": 12 }
    ```

- Request:
  ```JSON
  {
    "password": "thePasswordToHash!" // ,
    // "options": options
  }
  ```

- Response:
  ```JSON
  { "hash": "theResultingHash" }
  ```

#### Verify

- Endpoints:
  - `/argon2/verify`
  - `/bcrypt/verify`

- Request:
  ```JSON
  {
    "hash": "passwordHash",
    "password": "plainTextPassword"
  }
  ```

- Response:
  ```JSON
  { "result": true|false }
  ```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
