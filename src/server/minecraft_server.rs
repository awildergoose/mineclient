use std::{cell::RefCell, io::Error, rc::Rc};

use crate::{
    comm::{
        connection_listener::ConnectionListener, server_listener::ServerListener,
        socket_connection::SocketConnection, socket_server::SocketServer,
    },
    server::client::Client,
};

pub type SocketConnectionRef = Rc<RefCell<SocketConnection>>;
pub type ClientRef = Rc<RefCell<Client>>;

pub struct MinecraftServer {
    socket: RefCell<SocketServer>,
    client_map: RefCell<Vec<(SocketConnectionRef, ClientRef)>>,
    clients: RefCell<Vec<Rc<RefCell<Client>>>>,
}

impl MinecraftServer {
    pub fn new(ip: [u8; 4], port: u16) -> Result<Rc<Self>, Error> {
        let server = Rc::new(Self {
            socket: RefCell::new(SocketServer::new(ip, port, None)?),
            client_map: RefCell::new(vec![]),
            clients: RefCell::new(vec![]),
        });

        server
            .socket
            .borrow_mut()
            .set_server_listener(Some(Rc::clone(&server)));

        Ok(server)
    }

    pub fn disconnect(&self, client: &Client) {
        self.client_map
            .borrow_mut()
            .retain(|c| *c.1.borrow() != *client);
        self.clients.borrow_mut().retain(|c| *c.borrow() != *client);
    }

    pub fn tick(&mut self) {
        if let Err(e) = self.socket.borrow_mut().tick() {
            eprintln!("server tick error: {e}");
        }
    }
}

impl ServerListener for MinecraftServer {
    fn client_connected(self: Rc<Self>, socket: SocketConnectionRef) {
        let client = Client::new(Rc::clone(&self), socket.clone());
        self.clients.borrow_mut().push(client.clone());
        self.client_map.borrow_mut().push((socket, client));
    }

    fn client_exception(
        self: Rc<Self>,
        socket: SocketConnectionRef,
        error: std::sync::Arc<dyn std::error::Error>,
    ) {
        let binding = self.client_map.borrow();
        let client = binding.iter().find(|c| *c.0.borrow() == *socket.borrow());

        if let Some(client) = client {
            client.1.borrow_mut().handle_exception(error);
        } else {
            eprintln!("failed to find client to catch exception: {error:?}");
        }
    }
}
