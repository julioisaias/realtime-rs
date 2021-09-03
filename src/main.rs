mod supabase;

use crate::supabase::Client;

fn main() {
    let url = "wss://abc.supabase.co";
    let api = "zZaAHGQPi1hZ4nJy5CofZRIP686x_8j5_YzFYwEdWNk";

    let client = Client::new(url);
    let resp = client.apikey(api).channel("realtime:*");
    resp.connect();
}
