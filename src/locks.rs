use std::{error::Error, sync::Mutex};

use log::info;

pub fn lock_test() -> Result<(), Box<dyn Error>> {
    let m = Mutex::new(5);
    {
        let mut val = m.lock().unwrap();
        *val += 1;
    }
    info!("mutex = {:?}", m);
    info!("val = {}", *m.lock().unwrap());
    Err("lock test error".into())
}
