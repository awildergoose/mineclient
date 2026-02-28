use std::{error::Error, sync::Arc};

pub trait ConnectionListener {
    fn handle_exception(&self, exception: Arc<dyn Error>);
    fn command(&self, var1: u8, var2: usize, var3: Vec<u8>);
}
