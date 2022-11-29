#[allow(unused)]
struct Stack<T> {
    maxsize: i64,
    top: i64,
    items: Vec<T>,
}

#[allow(unused)]
impl<T> Stack<T> {
    fn push(&mut self, elem: T) {
        self.items.push(elem);
    }
    fn pop(&mut self) -> Option<T> {
        // let elem = self.items.get(0).expect("Failed to get first element").clone();
        // let elem = self.items.get(0).expect("Failed to get first element");
        // elem
        self.items.pop()
    }
}