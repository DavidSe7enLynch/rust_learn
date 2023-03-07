use log::info;
use std::any::Any;
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
    let vals = vec!["hello", "hi", "you", "world"];

    let h = thread::spawn(move || -> Result<(), SendError<&str>>{
        for val in vals {
            info!("sent: {}", val);
            tx.send(val)?;
            thread::sleep(Duration::from_micros(1));
        }
        Ok(())
    });
    rx.iter().for_each(|recv| {
        info!("received: {}", recv);
    });
    h.join().map_err(|_| "join error")??;
    Ok(())
}
