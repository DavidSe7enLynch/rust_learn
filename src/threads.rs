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
    let vals = vec!["hello", "hi", "you", "world"];

    let h = thread::spawn(move || -> Result<(), SendError<&str>> {
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

pub fn mpsc_test() -> Result<(), Box<dyn Error>> {
    let (tx0, rx) = mpsc::channel();
    let tx1 = tx0.clone();

    let h0 = thread::spawn(move || -> Result<(), SendError<&str>> {
        let vals = vec!["hello", "hi", "you", "world"];
        for val in vals {
            info!("thread 0 sent: {}", val);
            tx0.send(val)?;
            thread::sleep(Duration::from_micros(1));
        }
        Ok(())
    });

    let h1 = thread::spawn(move || -> Result<(), SendError<&str>> {
        let vals = vec!["hello", "hi", "you", "world"];
        for val in vals {
            info!("thread 1 sent: {}", val);
            tx1.send(val)?;
            thread::sleep(Duration::from_micros(1));
        }
        Ok(())
    });

    rx.iter().for_each(|recv| {
        info!("received: {}", recv);
    });
    
    h0.join().map_err(|_| {"thread 0 join error"})??;
    h1.join().map_err(|_| {"thread 1 join error"})??;
    // Ok(())
    Err("mpsc test err".into())
}
