mod states;

// use states;

struct Post {
    state: Option<Box<dyn states::State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(states::Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, msg: &str) {
        self.content.push_str(msg);
    }
    pub fn request_review(&mut self) {
        if let Some(state) = self.state {
            self.state = Some(state.request_review());
        }
    }
}
