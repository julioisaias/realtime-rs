# A basic and initial implementation of realtime client for supabase, build from scratch

## How to use

```rust
let url = "wss://qfdzxvjjpsbaiqhrehdda.supabase.co";
let api = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.zZaAHGQPi1hZ4nJy5CofZRIP686x_8j5_YzFYwEdWNk";


let client = Client::new(url);
let resp = client.apikey(api).channel("realtime:*");
    resp.connect();
```rust

# Dependencies

```toml
rand = "0.8.4"
native-tls = "0.2.7"
rustls = "0.19.1"
url = "2.2.2"
base64 = "0.13.0"
```toml

## License

MIT or GPL-3.0