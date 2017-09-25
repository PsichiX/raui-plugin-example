use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use std::ops::DerefMut;
use std::io::{Result, Error, ErrorKind};
use raui::server::*;

// DISCLAIMER:
// I really hate the idea of making singleton, but this is probably the only way to make
// server instance accessible across all API functions without passing references or even pointers.

#[derive(Clone)]
pub struct ServerSingleton {
    inner: Arc<Mutex<Server>>
}

impl ServerSingleton {

    pub fn perform<T>(&mut self, action: &mut FnMut(&mut Server) -> Result<T>) -> Result<T> {
        if let Ok(mut s) = self.inner.try_lock() {
            return action(s.deref_mut());
        }

        Err(Error::new(ErrorKind::Other, "Server instance is currently used in another place!"))
    }

}

pub fn server() -> ServerSingleton {
    static mut INSTANCE: *const ServerSingleton = 0 as *const ServerSingleton;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let instance = ServerSingleton {
                inner: Arc::new(Mutex::new(Server::new()))
            };

            INSTANCE = mem::transmute(Box::new(instance));
        });

        (*INSTANCE).clone()
    }
}
