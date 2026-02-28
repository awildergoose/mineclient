use std::{error::Error, sync::Arc};

pub trait ConnectionListener {
    fn handle_exception(&mut self, exception: Arc<dyn Error>);
    fn command(&mut self, cmd: u8, remaining: usize, data: Vec<u8>);
}
