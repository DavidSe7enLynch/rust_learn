use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use log::info;

pub fn lock_test() -> Result<(), Box<dyn Error>> {
    let m = Mutex::new(5);
    {
        let mut val = m.lock().map_err(|_| "mutex err -1")?;
        *val += 1;
    }
    info!("mutex = {:?}", m);

    let val = *m.lock().map_err(|_| "mutex err 0")?;
    info!("val = {}", *m.lock().map_err(|_| "mutex err 1")?);
    info!("val = {}", val);

    Err("lock test error".into())
}

pub fn lock_thread_test() -> Result<(), String> {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || -> Result<(), String> {
            let mut val = c.lock().map_err(|e| e.to_string())?;
            *val += 1;
            Ok(())
        });
        handlers.push(h);
    }
    for h in handlers {
        h.join()
            .map_err(|_| "join err")?
            .map_err(|e| format!("thread err: {e}"))?;
    }
    info!("counter = {}", *counter.lock().unwrap());
    Ok(())
}
