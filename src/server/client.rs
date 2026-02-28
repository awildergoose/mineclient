use std::{cell::RefCell, rc::Rc};

use crate::{
    comm::{connection_listener::ConnectionListener, socket_connection::SocketConnection},
    server::minecraft_server::MinecraftServer,
};

pub struct Client {
    server: Rc<MinecraftServer>,
    #[allow(dead_code)]
    connection: Rc<RefCell<SocketConnection>>,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        *self.connection.borrow() == *other.connection.borrow()
    }
}

impl Eq for Client {}

impl Client {
    #[must_use]
    pub fn new(
        server: Rc<MinecraftServer>,
        connection: Rc<RefCell<SocketConnection>>,
    ) -> Rc<RefCell<Self>> {
        let client = Rc::new(RefCell::new(Self {
            server,
            connection: connection.clone(),
        }));

        connection
            .borrow_mut()
            .set_connection_listener(Box::new(client.clone()));

        client
    }

    pub fn disconnect(&self) {
        self.server.disconnect(self);
    }
}

impl ConnectionListener for Client {
    fn handle_exception(&mut self, _exception: std::sync::Arc<dyn std::error::Error>) {
        self.disconnect();
    }

    fn command(&mut self, _cmd: u8, _remaining: usize, _data: Vec<u8>) {}
}
