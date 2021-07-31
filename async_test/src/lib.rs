async fn hello_world() {
    println!("hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn block_on_future_test() {
        let future = hello_world(); // Nothing is printed
        block_on(future); // `future` is run and "hello, world!" is printed
    }
}
