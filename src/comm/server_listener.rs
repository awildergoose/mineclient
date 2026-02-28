use crate::comm::socket_connection::SocketConnection;
use std::{error::Error, sync::Arc};

pub trait ServerListener {
    fn client_connected(var1: SocketConnection);
    fn client_exception(var1: SocketConnection, var2: Arc<dyn Error>);
}
