use crate::comm::socket_connection::SocketConnection;
use std::{cell::RefCell, error::Error, rc::Rc, sync::Arc};

pub trait ServerListener {
    fn client_connected(self: Rc<Self>, socket: Rc<RefCell<SocketConnection>>);
    fn client_exception(
        self: Rc<Self>,
        socket: Rc<RefCell<SocketConnection>>,
        error: Arc<dyn Error>,
    );
}
