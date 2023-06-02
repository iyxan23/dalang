## Auth Frontend

Not just a frontend, it has a small backend that connects to the auth service.

Developed with rust/leptos.

Application environment variables:

-   `HOMEPAGE`

    The url where users would get redirected to after they've logged in.

-   `AUTH_GRPC`

    The endpoint where the auth gRPC service is located.

-   `HOST`

    The host where the auth and other frontends serves on.

Example

```
HOMEPAGE="/home"
AUTH_GRPC="10.0.2.4:3333"
HOST="example.com"
```

## Running the frontend

`cargo leptos watch`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
frontend_auth
site/
```

Set the following enviornment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="frontend_auth"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.
