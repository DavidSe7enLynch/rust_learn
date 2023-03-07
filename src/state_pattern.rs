mod states;

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
        self.state = Some(self.state.take().unwrap().request_review());
    }
    pub fn approve(&mut self) {
        self.state = Some(self.state.take().unwrap().approve());
    }
    pub fn content(&self) -> &str {
        self.state.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn post() {
        
    }
}