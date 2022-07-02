#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    async fn hello_world() {
        println!("hello, world!");
    }

    #[test]
    fn block_on_future_test() {
        let future = hello_world(); // Nothing is printed
        block_on(future); // `future` is run and "hello, world!" is printed
    }

    use async_std::task;
    use std::time::Duration;

    #[test]
    fn test_aynsc_main() {
        let future = async_main();
        block_on(future);
    }

    async fn async_main() {
        print_for_five("await 1").await;

        let async_one = print_for_five("async 1");
        let async_two = print_for_five("async 2");

        futures::join!(async_one, async_two);
    }

    async fn print_for_five(msg: &str) {
        for _ in 0..5 {
            task::sleep(Duration::from_secs(1)).await;
            println!("one second has passed: {}", msg)
        }
    }
}
