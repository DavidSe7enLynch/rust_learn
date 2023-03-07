use std::{thread, sync::mpsc};
use log::info;

pub fn move_test() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        info!("v contains {:?}", v);
    });
    handle.join().unwrap();
}

pub fn channel_test() -> Result<(), ()> {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || -> Result<(), ()> {
        tx.send("hello world")?;
    });
    let recv_str = rx.recv()?;
}