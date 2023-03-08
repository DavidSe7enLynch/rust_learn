mod states;

pub struct Post {
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
        self.state.as_ref().unwrap().content(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn post() {
        let mut post = Post::new();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        post.add_text("hello world");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("hello world", post.content());

        post.request_review();
        assert_eq!("hello world", post.content());
    }
}
