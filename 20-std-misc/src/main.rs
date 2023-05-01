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
        // consume the iterator here to start the threads
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

#[allow(clippy::needless_collect)]
fn thread_map_reduce() {
    use std::thread;
    use std::thread::JoinHandle;

    const MAX_THREADS: usize = 12;

    let data = "
        869 67897737416471853297327050364959
        11861322575564723963297542624962850
        70856234701860 851 907960690014725639
        3839796670710609 4172783238747669219
        523807952578882365254593033303028 37
        58495327135744041048897885734297812
        69920216438980873548808413720956532
        1627842 4 6374 52 589860345374828574668
    ";
    let parts: Vec<&str> = data.split_whitespace().collect();
    // Build a nested list of threads, but DO NOT consume them
    let nested_threads = parts.chunks(MAX_THREADS).map(|xs| {
        xs.iter().enumerate().map(|(i, &x)| {
            thread::spawn(move || {
                println!("thread {i} executing for {x}");

                x.chars()
                    .map(|c| c.to_digit(10).expect("expected digit"))
                    .sum()
            })
        })
    });
    let total: u32 = nested_threads
        .into_iter()
        .enumerate()
        .inspect(|(i, _)| println!("\n---\nthread group {i} running...\n---"))
        .flat_map(|(_, xs)| {
            // Consume each nested thread synchronously so that we never have
            // more than MAX_THREADS running concurrently
            xs.collect::<Vec<JoinHandle<u32>>>()
                .into_iter()
                .map(|x| x.join().unwrap())
        })
        .sum();

    println!("\ntotal: {total}");
    println!()
}

// `main` is the main thread...
fn main() {
    thread_example();
    thread_map_reduce();
}
