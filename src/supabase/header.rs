use base64;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub struct Header {
    get: String,
    host: String,
    connection: String,
    upgrade: String,
    websocket_version: String,
    websocket_key: String,
}

trait Fields {
    fn get(&self) -> String;
    fn host(&self) -> String;
    fn connection(&self) -> String;
    fn upgrade(&self) -> String;
    fn websocket_version(&self) -> String;
    fn websocket_key(&self) -> String;
}

impl Fields for Header {
    fn get(&self) -> String {
        format!("GET {}{}", self.get, nl())
    }
    fn host(&self) -> String {
        format!("Host: {}{}", self.host, nl())
    }
    fn connection(&self) -> String {
        format!("Connection: {}{}", self.connection, nl())
    }
    fn upgrade(&self) -> String {
        format!("Upgrade: {}{}", self.upgrade, nl())
    }
    fn websocket_version(&self) -> String {
        format!("Sec-WebSocket-Version: {}{}", self.websocket_version, nl())
    }
    fn websocket_key(&self) -> String {
        format!("Sec-WebSocket-Key: {}{}{}", self.websocket_key, nl(), nl())
    }
}

impl Header {
    pub fn new(url: String, api_key: String) -> String {
        let h = Header {
            get: format!(
                "{}/realtime/v1/websocket?apikey={}&vsn=2.0.0 HTTP/1.1",
                url, api_key
            ),
            host: "supabase.co".to_string(),
            connection: "Upgrade".to_string(),
            upgrade: "websocket".to_string(),
            websocket_version: "13".to_string(),
            websocket_key: get_web_socket_key(),
        };
        
        /* All headers are concatenated into a string */
        format!(
            "{}{}{}{}{}{}",
            h.get(),
            h.host(),
            h.connection(),
            h.upgrade(),
            h.websocket_version(),
            h.websocket_key()
        )
    }
}

/* A key is generated for the websocket-key */
pub fn get_web_socket_key() -> String {
    let n1: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    base64::encode(n1.as_bytes())
}

fn nl() -> String {
    String::from("\r\n")
}
