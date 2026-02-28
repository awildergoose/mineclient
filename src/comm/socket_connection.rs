use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use crate::{comm::connection_listener::ConnectionListener, java::JBoolean};

pub static BUFFER_SIZE: usize = 131_068;

pub struct SocketConnection {
    socket: TcpStream,
    write_buffer: Vec<u8>,
    connection_listener: Option<Arc<dyn ConnectionListener>>,
    connected: JBoolean,
}

impl SocketConnection {
    pub fn new(
        ip: String,
        port: u16,
        connection_listener: Arc<dyn ConnectionListener>,
    ) -> Result<Self, Error> {
        let socket = TcpStream::connect((ip, port))?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            write_buffer: vec![0; BUFFER_SIZE],
            connection_listener: Some(connection_listener),
            connected: true,
        })
    }

    #[must_use]
    pub fn from_socket(socket: TcpStream) -> Self {
        Self {
            socket,
            write_buffer: vec![0; BUFFER_SIZE],
            connection_listener: None,
            connected: true,
        }
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        self.connected = false;
        self.write_buffer = vec![0; BUFFER_SIZE];
        self.socket.shutdown(std::net::Shutdown::Both)
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        match self.socket.write(&self.write_buffer) {
            Ok(n) => {
                self.write_buffer.drain(..n);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }

        let mut temp = vec![0u8; BUFFER_SIZE].into_boxed_slice();

        match self.socket.read(&mut temp) {
            Ok(0) => {
                // connection closed
            }
            Ok(n) => {
                if n > 0 {
                    let opcode = temp[0];

                    if let Some(listener) = &self.connection_listener {
                        listener.command(opcode, n, temp[..n].to_vec());
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    #[must_use]
    pub const fn write_buffer(&self) -> &Vec<u8> {
        &self.write_buffer
    }

    #[must_use]
    pub const fn write_buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.write_buffer
    }

    #[must_use]
    pub const fn is_connected(&self) -> JBoolean {
        self.connected
    }
}
