## realtime-rs

A basic and initial implementation of realtime client for [supabase.io](https://supabase.io) written in `Rust`, build from scratch using pure websocket protocol.

## How to use

In the `main.rs` file you can fill in the following data provided by supabase.io:

```rust
let url = "wss://abc.supabase.co";
let api = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.zZaAHGQPi1hZ4nJy5CofZRIP686x_8j5_YzFYwEdWNk";


let client = Client::new(url);
let resp = client.apikey(api).channel("realtime:*");
    resp.connect();
```
Then run with `cargo run`. From here you can listen to all the changes made in the cloud database.

## Testing

This implementation can be tested using `cargo test` to run tests and `cargo bench` to run bench tests.

## Dependencies

Add this to your Cargo.toml:

```toml
rand = "0.8.4"
native-tls = "0.2.7"
rustls = "0.19.1"
url = "2.2.2"
base64 = "0.13.0"
```

## Disclaimer

This is not a crate. This is a basic implementation so some functionality may be missing.

## Contribution

Realtime-es is free open source software developed in my (little) spare time. If you liked the implementation and would like to support further development, please consider contributing, suggesting, or forking.

## License

MIT
