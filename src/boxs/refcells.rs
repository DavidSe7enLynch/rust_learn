trait MessageSender {
    fn send(&self, msg: &str); 
}

struct LimitChecker<'a, T>
where T: MessageSender
{
    message_sender: &'a T,
    value: usize,
    max: usize,
}

