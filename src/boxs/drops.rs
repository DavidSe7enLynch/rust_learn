use log::info;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        info!("drop CustomSmartPointer: {}", self.data);
    }
}

pub fn drop_test() {
    let a = CustomSmartPointer {
        data: "hello".to_string(),
    };
    drop(a);
}
