use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(stream: Result<TcpStream, std::io::Error>, processor: &mut Processor) -> Result<(), Error> {
    match stream {
        Ok(mut stream_unwrapped) => {
            let mut s = [0; 1024];
            let len = stream_unwrapped.read(&mut s)?;       
            let command = String::from_utf8_lossy(&s[0..len]).to_string();
            let result = processor.process(command);
            let response = match result {
                Some(some) => some,
                None => "Ok".to_owned(),
            };
            stream_unwrapped.write(format!("{}\r\n", response).as_bytes()).unwrap();
            stream_unwrapped.flush().unwrap();
            Ok(())
        }
        Err(e) => Err(e),
    }
}

struct Processor {
    db: KeyValueStore,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            db: KeyValueStore::new(),
        }
    }
    fn process(&mut self, cmd: String) -> Option<String> {
        let mut args = cmd.split_ascii_whitespace();
        match args.next() {
            Some(arg) => match arg {
                "set" => {
                    let key = args.next().unwrap();
                    let value = args.next().unwrap();
                    self.db.set(key, value);
                    Some("Ok".to_string())
                }
                "get" => {
                    let key = args.next().unwrap();
                    let val = self.db.get(key);
                    match val {
                        Some(val) => Some(val.to_owned()),
                        None => None,
                    }
                },
                _ => Some("Unknown command".to_owned()),
            },
            None => todo!(),
        }
    }
}

struct KeyValueStore {
    // Lets only store strings for now
    db: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore { db: HashMap::new() }
    }
    pub fn set(&mut self, key: &str, value: &str) {
        match self.db.insert(key.to_owned(), value.to_owned()) {
            Some(_) => println!("Key updated"),
            None => println!("Key added"),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.db.get(key)
    }
}

fn main() -> Result<(), std::io::Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)?;
    let mut processor = Processor::new();
    for stream in listener.incoming() {
        match handle_client(stream, &mut processor) {
            Ok(_) => {},
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
