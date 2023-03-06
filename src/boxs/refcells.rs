trait MessageSender {
    fn send(&self, msg: &str);
}

struct LimitChecker<'a, T>
where
    T: MessageSender,
{
    message_sender: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitChecker<'a, T>
where
    T: MessageSender,
{
    pub fn new(message_sender: &'a T, max: usize) -> LimitChecker<'a, T> {
        LimitChecker {
            message_sender,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let ratio = (self.value as f64 / self.max as f64 * 100.0) as usize;
        match ratio {
            0..=75 => {
                self.message_sender.send("<= 75%");
            }
            76..=90 => {
                self.message_sender.send("> 75%, <= 90%");
            }
            _ => {
                self.message_sender.send("> 90%");
            }
        }
    }
}



