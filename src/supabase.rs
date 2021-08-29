use std::io::{ BufReader, Read, Write };
use std::net::TcpStream;
use std::net::{ SocketAddr, ToSocketAddrs };

use native_tls::{ TlsConnector, TlsStream };
use url::{ Url };
use rand::RngCore;

mod header;
use header::Header;

pub struct Client {
    url: String,
    apikey: Option<String>,
    channel: Option<String>
}

impl Client {

    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Client {
            url: url.into(),
            apikey: None,
            channel: None
        }
    }

    pub fn apikey<T>(mut self, api_key: T) -> Self
    where
        T: Into<String>,
    {
        self.apikey = Some(api_key.into());
        self
    }

    pub fn channel<T>(mut self, channel: T) -> Self
    where
        T: Into<String>,
    {
        self.channel = Some(channel.into());
        self
    }

    pub fn uri(&self) -> String {
        let mut owned_string: String = host( &*self.url ).to_string();
                owned_string.push_str(":");
                owned_string.push_str(&port(&*self.url));
         return owned_string;
    }

    pub fn connect(&self) -> ()
    
    {

        let host = host( &*self.url ).to_string();
        let mut into_iter = self.uri().to_socket_addrs().unwrap();
        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(SocketAddr::from(into_iter.nth(0).unwrap())).unwrap();
        let mut stream_ssl = connector.connect(&host, stream).unwrap();


        let headers = Header::new(self.url.to_owned().to_string(),  self.apikey.to_owned().unwrap() );
        let response = send_data(&mut stream_ssl, headers.as_bytes());
        println!("Header response: {:?}\n", &response);

        /*------------------------------ */

        let line = format!("{}{}{}", r#"["1", "1", ""#, self.channel.as_ref().unwrap(), r#"", "phx_join", {}]"# );
        let encrypted_data = mask_payload(line);
        let response = send_data(&mut stream_ssl, &encrypted_data);
        println!("Channel join response: {:?}\n", &response);

        loop {
            let mut cc: Vec<u8> = vec![0u8; 1024];
            let mut reader = BufReader::new(&mut stream_ssl);

            reader.read(&mut cc).expect("Could no read into buffer");

            let res = String::from_utf8_lossy(&cc);
            let res = res.trim_matches(char::from(0));
            
            println!("cc: {:?}\n", res);
            // println!("cc: {:?}", String::from_utf8(cc));
        }


    }

}


fn host(url: &str) -> String {
    Url::parse(&url).unwrap().host_str().unwrap().to_string()
}

fn port(url: &str) -> String{
    match &*Url::parse(&url).unwrap().scheme().to_string() {
        "wss" => "443".to_string(),
        "ws"  => "80".to_string(),
        _     => "80".to_string()
    }
}


fn send_data<'a>(stream_ssl: &'a mut TlsStream<TcpStream>, buf: &'a [u8]) -> String {
    stream_ssl.write_all(&buf).expect("Failed to write to server"); // Escribir la cabecera
    let mut buffer: Vec<u8> = vec![0u8; 1024];
    let mut reader = BufReader::new(stream_ssl);
            reader.read(&mut buffer).expect("Could no read into buffer");
    let res = String::from_utf8_lossy(&buffer);
        res.trim_matches(char::from(0)).to_owned()
}


fn mask_payload(line: String) -> Vec<u8> {

    let mut data: Vec<u8> = Vec::new();
        let mut fin = vec![0x81];
        let mut payload_length_hex : Vec<u8> = Vec::new();
        let mut payload_xored = vec![];

        let mut masking_key = [0u8; 4];
        rand::thread_rng().fill_bytes(&mut masking_key);

        data.append(&mut fin);

        let payload_length = &line.chars().count();
        payload_length_hex.push((*payload_length as u8) + 0x81);

        for (i, c) in line.chars().enumerate() {
            payload_xored.push((c as u8) ^ masking_key[i % 4]);
            if i == payload_length - 1 {
                payload_xored.push((0xa as u8) ^ masking_key[(i + 1) % 4])
            }
        }

        data.append(&mut payload_length_hex);
        data.append(&mut masking_key.iter().cloned().collect());
        data.append(&mut payload_xored.iter().cloned().collect());
        return data;
}