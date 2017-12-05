pub trait Show {
    fn show(self) -> String;
}

impl Show for i32 {
    fn show(self) -> String {
        self.to_string()
    }
}

impl Show for String {
    fn show(self) -> String {
        self
    }
}
