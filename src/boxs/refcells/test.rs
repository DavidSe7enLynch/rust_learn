use std::cell::RefCell;

use super::*;

struct MockMessageSender {
    msgs_sent: RefCell<Vec<String>>,
}

impl MessageSender for MockMessageSender {
    fn send(&self, msg: &str) {
        self.msgs_sent.borrow_mut().push(msg.to_string());
    }
}

#[test]
fn limit_checker() {
    let mock_msg_sender = MockMessageSender {
        msgs_sent: RefCell::new(vec![]),
    };
    let mut limit_checker = LimitChecker::new(&mock_msg_sender, 22);
    limit_checker.set_value(20);
    assert_eq!(mock_msg_sender.msgs_sent.borrow().len(), 1);
}
