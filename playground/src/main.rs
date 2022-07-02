use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // Deref coercion 隐式转换: &String => &str, compile time, no runtime penalty
    let s = String::from("Rust");
    hello(&s);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    hello(&*m);
    hello(&m); // Deref coercion twice: &MyBox<String> => &String, &String => &str, compile time, no runtime penalty
}
