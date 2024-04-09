use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::aynchro::asynchro;
use crate::{debug, info};

pub struct ServerState {
    pub manager: Arc<Mutex<ClientManager>>,
    pub max_sockets: u8,
    pub require_auth: bool,
    pub secure: bool,
    pub domain: String,
}

pub struct ClientManager {
    pub clients: HashMap<String, Arc<Mutex<Client>>>,
    pub _tunnels: u16,
    pub default_max_sockets: u8,
}

pub struct Client {
    pub available_sockets: Arc<Mutex<Vec<TcpStream>>>,
    pub port: Option<u16>,
    pub max_sockets: u8,
}

impl Client {
    pub fn new(max_sockets: u8) -> Self {
        Client {
            available_sockets: Arc::new(Mutex::new(vec![])),
            port: None,
            max_sockets,
        }
    }

    pub fn listen(&mut self) -> io::Result<u16> {
        let listener = TcpListener::bind("0.0.0.0")?;
        let port = listener.local_addr()?.port();
        self.port = Some(port);

        let sockets = self.available_sockets.clone();
        let max_sockets = self.max_sockets;

        // Change the usage of `spawn` and ensure that it is compatible with such tasks
        asynchro::spawn(move || {
            loop {
                match listener.accept() {
                    Ok((socket, addr)) => {
                        info!("new client connection: {:?}", addr);

                        let mut sockets = sockets.lock().unwrap();

                        debug!("Sockets length: {}", sockets.len());
                        if sockets.len() < max_sockets as usize {
                            debug!("Add a new socket, max: {}", max_sockets);
                            sockets.push(socket)
                        }
                    }
                    Err(e) => info!("Couldn't get client: {:?}", e),
                }
            }
        });

        Ok(port)
    }

    pub fn take(&mut self) -> Option<TcpStream> {
        let mut sockets = self.available_sockets.lock().unwrap();
        sockets.pop()
    }
}

impl ClientManager {
    pub fn new(max_sockets: u8) -> Self {
        ClientManager {
            clients: HashMap::new(),
            _tunnels: 0,
            default_max_sockets: max_sockets,
        }
    }

    pub fn put(&mut self, url: String) -> io::Result<u16> {
        let client = Arc::new(Mutex::new(Client::new(self.default_max_sockets)));
        self.clients.insert(url, client.clone());

        let mut client = client.lock().unwrap();
        client.listen()
    }
}
