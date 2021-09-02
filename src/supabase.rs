use std::borrow::BorrowMut;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::net::{ SocketAddr, ToSocketAddrs };

use std::time::Duration;
use std::sync::{ Arc, Mutex };

use native_tls::{ TlsConnector, TlsStream };

use url::{ Url };
use rand::RngCore;


mod header;
use header::Header;

use std::thread;
use std::sync::mpsc;

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


        //let stream = TcpStream::connect(SocketAddr::from(into_iter.nth(0).unwrap()) ).unwrap();
        let stream = TcpStream::connect_timeout(&SocketAddr::from(into_iter.nth(0).unwrap()), Duration::new(120, 0)).unwrap();
        
        stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

        let mut stream_ssl = connector.connect(&host, stream).unwrap();


        let headers = Header::new(self.url.to_owned().to_string(),  self.apikey.to_owned().unwrap() );
        let response = send_data(&mut stream_ssl, headers.as_bytes());
        println!("Header response: {:?}\n", &response);

        /*------------------------------ */

        let line = format!("{}{}{}", r#"["1", "1", ""#, self.channel.as_ref().unwrap(), r#"", "phx_join", {}]"# );
        let encrypted_data = mask_payload(line);
        let response = send_data(&mut stream_ssl, &encrypted_data);
        println!("Channel join response: {:?}\n", &response);


        //let sender_ref = Arc::new(Mutex::new(stream_ssl));
        //let sender = Arc::new(Mutex::new(stream_ssl));
        //let sender_ref = Mutex::new(stream_ssl);
        //let sender_ref = Arc::clone(&sender);

        let sender = Arc::new(Mutex::new(stream_ssl));
        
        let sender_ref = Arc::clone(&sender);

        // thread::spawn(move || {
        //     println!("otro thread");

        // });

        // let ping = mask_payload("ping".to_string());

        // loop {
        //     let heart_beat =  send_data(&mut sender_ref.lock().unwrap(), &ping);
        //     thread::sleep(Duration::from_secs(5));
        //     println!("RESPONSE: {}{:?}\n", c, heart_beat);
        //     c = c + 1;
        // }

        let (tx, rx) = mpsc::channel();

        let t =thread::spawn(move || {
            let mut c = 0;
            //let mut s = sender.lock().unwrap();

            loop {
                c=c+1;
                //println!("-------------HEART BEAT RESPONSE (SIN VARIABLES)--------------{}", c);
                thread::sleep(Duration::from_secs(5));
                //tx.send("corazon");
                tx.send(encrypted_data.clone()).unwrap();

                // println!("Joining HEART BEAT...{}", c);
                // //&stream_ssl.flush();
                
                
                //let s = &mut *s;
                
                
                
                // let mut buffer: Vec<u8> = vec![0u8; 1024];
                // s.write_all(&encrypted_data).expect("Failed to write HEART BEAT to server");
                // //let mut reader = BufReader::new( s);
                // s.read(&mut buffer).expect("Could no read into buffer");
                // let response = String::from_utf8_lossy(&buffer);
                // let r = response.trim_matches(char::from(0)).to_owned();

                // //sender.is_poisoned();
                
                
                // 
                // println!("HEART BEAT RESPONSE: {} - {}", c, r);
                // buffer.clear();
                 //s.flush().unwrap();
                 //std::mem::drop(s);
            }
        });

        //t.join().unwrap();

        

        


        //let u = thread::spawn(move || {
                let mut d = 0;
                let mut sr = sender_ref.lock().unwrap();
                loop {

                    //println!("Joining MAIN...\n");
                    let msg = rx.recv().unwrap();
                    //println!("RX: {}", msg);
                    
                    //let response = read_incoming(&mut sender.lock().unwrap());
                    //thread::sleep(Duration::from_secs(1));
                    let sr = &mut *sr;
                    
                    
                    let mut buffer: Vec<u8> = vec![0u8; 1024];
                    
                    sr.write_all(&msg).expect("Failed to write HEART BEAT to server");

                    //sr.read(&mut buffer).unwrap();

                    let _ = match sr.read(&mut buffer) {
                        Err(e) => {
                            match e.kind() {
                                io::ErrorKind::WouldBlock => {
                                    //println!("would have blocked");
                                    //break;
                                },
                                _ => panic!("Got an error: {}", e),
                            }
                        },
                        Ok(m) => {
                            let response = String::from_utf8_lossy(&buffer);
                            let r =    response.trim_matches(char::from(0));

                            //thread::sleep(Duration::from_secs(1));
                            
                            println!("--------SUPABASE--------: {:?} - {:?}\n", d, r);
                            
                            if m == 0 {
                                // doesn't reach here.
                                break;
                            }
                            ()
                        },

                    };
                    //let mut buffer_s = String::new();
                    /*
                    let mut reader = BufReader::new(sr);
                            reader.read(&mut buffer).expect("Could no read into buffer");

                    */
                    //&mut stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
                    
                    d=d+1;
                    

                    sr.flush().unwrap();
                    std::mem::drop(sr);

                }
        //});


        //m.join().expect("The k thread has panicked");

    }
}


// fn keep_alive(stream_ssl: Mutex<TlsStream<TcpStream>>) -> thread::JoinHandle<()>{
//     println!("entrando...");
//     return thread::spawn(move || {
//         loop {
//             thread::sleep(Duration::from_secs(5));
//             let mut sender = stream_ssl.lock().unwrap();
//             //sender.heartbeat();
//             let heart_beat =  &send_data(&mut sender, &vec![0x9]);
//             //let heart_beat = "kk";
//             println!("HEART BEAT RESPONSE: {:?}", &heart_beat.as_bytes());
//         }
//     });
// }


fn read_incoming<'a>(stream_ssl: &'a mut TlsStream<TcpStream>) -> String {

    let mut buffer: Vec<u8> = vec![0u8; 1024];
    let mut reader = BufReader::new(stream_ssl);
            reader.read(&mut buffer).expect("Could no read into buffer");
    let response = String::from_utf8_lossy(&buffer);
        response.trim_matches(char::from(0)).to_owned()
}



// fn read_incoming(stream_ssl: Mutex<TlsStream<TcpStream>>) -> String {
//     let mut buffer: Vec<u8> = vec![0u8; 1024];
//     let mut reader = BufReader::new(stream_ssl.into_inner().unwrap());
//             reader.read(&mut buffer).expect("Could no read into buffer");
//     let response = String::from_utf8_lossy(&buffer);
//         response.trim_matches(char::from(0)).to_owned()
// }



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
    println!("DATA SENT: {:?}", &buf);
    stream_ssl.write_all(&buf).expect("Failed to write to server");
    let mut buffer: Vec<u8> = vec![0u8; 1024];
    //let mut reader = BufReader::new(stream_ssl);
    stream_ssl.read(&mut buffer).expect("Could no read into buffer");
    let response = String::from_utf8_lossy(&buffer);
        response.trim_matches(char::from(0)).to_owned()
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

