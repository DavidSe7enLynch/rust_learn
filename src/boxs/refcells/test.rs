use log::debug;

use super::*;

struct MockMessageSender {
    msgs_sent: Vec<String>,
}

impl MessageSender for MockMessageSender {
    fn send(&self, msg: &str) {
        self.msgs_sent.push(msg.to_string());
    }
}

#[test]
fn limit_checker() {
    let mock_msg_sender = MockMessageSender { msgs_sent: vec![] };
    let mut limit_checker = LimitChecker::new(&mock_msg_sender, 22);
    limit_checker.set_value(20);
    assert_eq!(mock_msg_sender.msgs_sent.len(), 1);
}
