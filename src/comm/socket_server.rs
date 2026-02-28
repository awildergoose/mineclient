use std::{
    cell::RefCell,
    io::{Error, ErrorKind},
    net::{IpAddr, Ipv4Addr, TcpListener},
    rc::Rc,
    sync::Arc,
};

use crate::{
    comm::{server_listener::ServerListener, socket_connection::SocketConnection},
    server::minecraft_server::MinecraftServer,
};

pub type SocketServerConnection = Rc<MinecraftServer>;

pub struct SocketServer {
    server_listener: Option<SocketServerConnection>,
    socket: TcpListener,
    connections: Vec<Rc<RefCell<SocketConnection>>>,
}

impl SocketServer {
    pub fn new(
        ip: [u8; 4],
        port: u16,
        server_listener: Option<SocketServerConnection>,
    ) -> Result<Self, Error> {
        let addr = IpAddr::V4(Ipv4Addr::from_octets(ip));
        let socket = TcpListener::bind((addr, port))?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            server_listener,
            socket,
            connections: vec![],
        })
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        loop {
            match self.socket.accept() {
                Ok((client, _addr)) => {
                    client.set_nonblocking(true)?;
                    let socket = SocketConnection::from_socket(client);
                    if let Some(listener) = &self.server_listener {
                        listener.clone().client_connected(socket.clone());
                    }
                    self.connections.push(socket);
                }
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        break;
                    }

                    eprintln!("tcp accept error: {e:?}");
                    break;
                }
            }
        }

        self.connections.retain_mut(|connection| {
            let mut connection_mut = connection.borrow_mut();
            if !connection_mut.is_connected() {
                if let Err(e) = connection_mut.disconnect() {
                    eprintln!("failed to shutdown socket for client: {e}");
                }

                return false;
            }

            if let Err(e) = connection_mut.tick()
                && let Some(listener) = &self.server_listener
            {
                listener
                    .clone()
                    .client_exception(connection.clone(), Arc::new(e));
            }

            true
        });

        Ok(())
    }

    pub fn set_server_listener(&mut self, server_listener: Option<SocketServerConnection>) {
        self.server_listener = server_listener;
    }
}
