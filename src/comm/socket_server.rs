use std::{
    io::{Error, ErrorKind},
    net::{IpAddr, Ipv4Addr, TcpListener},
    sync::Arc,
};

use crate::comm::{server_listener::ServerListener, socket_connection::SocketConnection};

pub struct SocketServer {
    server_listener: Arc<dyn ServerListener>,
    socket: TcpListener,
    connections: Vec<SocketConnection>,
}

impl SocketServer {
    pub fn new(
        ip: [u8; 4],
        port: u16,
        server_listener: Arc<dyn ServerListener>,
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
                    self.server_listener.client_connected(&socket);
                    self.connections.push(socket);
                }
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        break;
                    }

                    println!("tcp accept error: {e:?}");
                    break;
                }
            }
        }

        self.connections.retain_mut(|connection| {
            if !connection.is_connected() {
                if let Err(e) = connection.disconnect() {
                    println!("failed to shutdown socket for client: {e}");
                }

                return false;
            }

            if let Err(e) = connection.tick() {
                self.server_listener
                    .client_exception(connection, Arc::new(e));
            }

            true
        });

        Ok(())
    }
}
