mod supabase;

use crate::supabase::Client;

fn main() {

    let url = "wss://qfdzxqspsbaiqhrehiia.supabase.co";
    let api = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyMTc0MjYwMywiZXhwIjoxOTM3MzE4NjAzfQ.zZaAHGQPi1hZ4nJy5CofZRIP686x_8j5_YzFYwEdWNk";


    let client = Client::new(url);
    let resp = client.apikey(api).channel("realtime:*");
        resp.connect();
    
}
