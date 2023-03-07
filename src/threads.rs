use log::info;
use std::error::Error;
use std::sync::mpsc::{self, SendError};
use std::thread;
use std::time::Duration;

pub fn move_test() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        info!("v contains {:?}", v);
    });
    handle.join().unwrap();
}

pub fn channel_test() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();

    let h = thread::spawn(move || {
        tx.send("hello world").unwrap_or_else(|e| {
            // return Err(e);
        });
        // Ok(())
    });

    // h.join().unwrap_or_else(|e| {return Err(e);});
    // thread::sleep(Duration::from_millis(1));
    info!("received: {}", rx.try_recv()?);

    Ok(())
}
