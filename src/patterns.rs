use log::info;

enum Message {
    Hello { val: i32, id: i32 },
}

pub fn pattern_test() {
    let msg = Message::Hello { val: 15, id: 1 };
    match msg {
        Message::Hello {
            val: val_test @ 1..=12,
            id,
        } if id > 0 && val_test != id => info!("1..=12: val = {}, id = {}", val_test, id),
        Message::Hello { val: 13, id: x } => info!("13, {}", x),
        Message::Hello { val: val_test, .. } => info!("val_test: {}", val_test),
    }
}
