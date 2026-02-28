use crate::comm::socket_connection::SocketConnection;
use std::{error::Error, sync::Arc};

pub trait ServerListener {
    fn client_connected(&self, var1: &SocketConnection);
    fn client_exception(&self, var1: &mut SocketConnection, var2: Arc<dyn Error>);
}
