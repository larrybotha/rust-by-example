fn thread_example() {
    use std::thread;
    use std::thread::JoinHandle;

    const NUM_THREADS: u32 = 10;

    #[allow(clippy::needless_collect)]
    let children = (0..NUM_THREADS)
        .map(|i| {
            thread::spawn(move || {
                println!("thread executing for iteration {}", &i);

                format!("result for this thread: {i}")
            })
        })
        .collect::<Vec<JoinHandle<String>>>();

    println!("\nnow waiting for each thread to complete...\n");

    children
        .into_iter()
        .map(|child| child.join()) // wait for the thread to complete
        // or
        //.map(JoinHandle::join)
        .map(|result| println!("{result:?}"))
        .for_each(drop);

    println!()
}

// `main` is the main thread...
fn main() {
    thread_example();
}
