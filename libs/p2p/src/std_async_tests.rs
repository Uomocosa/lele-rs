#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread, time::Duration};
    use futures::{executor::block_on, future, select};

    fn expensive_computation() -> i32 {
        thread::sleep(Duration::from_secs(2));
        println!("(finished expensive_computation)");
        1
    }

    #[test]
    fn scope_test_1() {
        println!("tesing ...");
        thread::scope(|s| {
            s.spawn(|| { expensive_computation() });
        });
        assert_eq!(2+2, 4);
    }

    #[test]
    fn scope_test_2() {
        println!("tesing ...");
        thread::scope(|s| {
            s.spawn(|| { expensive_computation() });
        });
        println!("finished scope!");
        assert_eq!(2+2, 4);
    }

    #[test]
    fn channel_test_1() {
        println!("tesing ...");
        let (sender, receiver) = channel();
        thread::spawn(move || {
            sender.send(expensive_computation()).unwrap();
        });
        println!("hello ...");
        println!("received: {:?}", receiver.recv().unwrap()); // this acts as an await!
        println!("... world!");
    }

    #[test]
    fn channel_test_2() {
        println!("tesing ... ");
        let (sender, receiver) = channel();
        println!("spawning thread ...");
        thread::spawn(move || {
            sender.send(expensive_computation()).unwrap();
        });
        println!("receiving thread ...");
        receiver.recv().unwrap();
        println!("finshed test.");
    }

    async fn select_faster_async_function(n: i32) -> i32 {
        let mut f1 = future::ready(n);
        let mut f2 = future::pending::<()>();

        let res = select! {
            a_res = f1 => a_res + 1,
            () = f2 => 0,
        };
        res
    }

    #[test]
    fn select_test_1() {
        println!("tesing ... ");
        let x = block_on(select_faster_async_function(4));
        assert_eq!(x, 5);
    }
}