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

```TS
const result = await (await env.CRYPTOFLARE.fetch($ENDPOINT, {
  method: 'POST',
  body: JSON.stringify($DATA),
})).json();
```

### API Specification

#### Hash

- Endpoints:
  - `/argon2/hash`, with available `Option`:
    ```TS
    {
      "time_cost": number, /* defaults to 2 */
      "memory_cost": number, /* defaults to 19 * 1024 = 19456 */
      "parallelism": number /* defaults to 1 */
    }
    ```
  - `/bcrypt/hash`, with available `Option`:
    ```TS
    { "work_factor": number /* defaults to 12 */ }
    ```

- Request:
  ```TS
  {
    "password": string,
    "options"?: Option
  }
  ```

- Response:
  ```TS
  { "hash": string }
  ```

#### Verify

- Endpoints:
  - `/argon2/verify`
  - `/bcrypt/verify`

- Request:
  ```TS
  {
    "hash": string,
    "password": string
  }
  ```

- Response:
  ```TS
  { "result": boolean }
  ```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
