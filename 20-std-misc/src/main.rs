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
        .for_each(|result| println!("{result:?}"));

    println!()
}

#[allow(clippy::needless_collect)]
fn thread_map_reduce() {
    use std::thread;

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
    let parts: Vec<_> = data.split_whitespace().collect();
    // Build a nested list of threads, but DO NOT consume them
    let nested_threads = parts.chunks(MAX_THREADS).map(|xs| {
        xs.iter().enumerate().map(|(i, &x)| {
            thread::spawn(move || {
                // note how execution order of threads is not necessarily in order
                println!(
                    "thread #{i} with id {:?} executing for {x}",
                    thread::current().id()
                );

                let sum: u32 = x
                    .chars()
                    .map(|c| c.to_digit(10).expect("expected digit"))
                    .sum();

                sum
            })
        })
    });
    let total: u32 = nested_threads
        .enumerate()
        .inspect(|(i, _)| println!("\n---\nthread group {i} running...\n---"))
        .flat_map(|(_, xs)| {
            // Consume each nested thread synchronously so that we never have
            // more than MAX_THREADS running concurrently
            xs.collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.join().unwrap())
        })
        .sum();

    println!("\ntotal: {total}");
    println!()
}

fn channel_example() {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;

    const NUM_THREADS: i32 = 3;

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut join_handles = Vec::new();

    for id in 0..NUM_THREADS {
        // for multiple senders, the sender can be cloned
        let thread_sender = tx.clone();
        // spawn a thread, creating a JoinHandle
        let join_handle = thread::spawn(move || {
            // send a message to the receiver, taking ownership of the
            // sender
            // We are queuing a message in the channel here
            thread_sender.send(id).unwrap();

            println!(
                "thread {:?} finished with value {id}",
                thread::current().id()
            );
        });

        // append the JoinHandle to the others
        join_handles.push(join_handle)
    }

    let mut received_ids = Vec::with_capacity(NUM_THREADS as usize);

    // Collect all the messages sent to the channel.
    // We need to loop over however many messages were sent - if we send
    // more messages, say by using thread_sender multiple times in each
    // spawned thread, then with this loop we'd only received the first
    // few messages
    for _ in 0..NUM_THREADS {
        println!("receiving");
        // push each message the channel receives onto our vector
        received_ids.push(rx.recv());
    }

    // Wait for every thread to complete its work before moving on
    for handle in join_handles {
        println!("joining {:?}", handle.thread().id());
        // wait for the thread to finish its work
        handle.join().expect("oops, child thread panicked!")
    }

    println!("\nreceived_ids: {received_ids:?}");
    println!()
}

fn path_example() {
    use std::path::{Path, PathBuf};

    let path = Path::new(".");

    println!("displayable path: {}\n", path.display());

    for child in path.read_dir().unwrap() {
        println!("child: {child:?}");
    }

    let mut new_path: PathBuf = path.join("foo").join("bar");

    println!("\nnew_path: {}", new_path.display());

    new_path.push("baz");
    println!("new_path: {}", new_path.display());
    new_path.push("my-awesome-filename.txt");
    println!("new_path: {}", new_path.display());

    new_path.set_file_name("change-that-filename.txt");
    println!("new_path: {}", new_path.display());

    println!(
        "metadata of README: {:#?}",
        Path::new("./README.md").metadata()
    );

    println!()
}

fn file_auto_close() {
    use std::fs::File;
    use std::path::Path;

    let readme = Path::new("./README.md");

    if readme.exists() {
        let file = match File::open(readme) {
            Ok(result) => result,
            Err(reason) => {
                panic!("unable to open {}: {reason}", readme.display());
            }
        };

        println!("file {}: {file:#?}", readme.display());
        println!(
            "about to drop variable 'file' {:p}, and close file...",
            &file
        )
    }

    println!("file closed!");
    println!()
}

fn file_read() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let readme = Path::new("./README.md");
    let mut file = match File::open(readme) {
        Ok(result) => result,
        Err(reason) => panic!("unable to open {}: {reason}", readme.display()),
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            println!("file read!")
        }
        Err(reason) => panic!(
            "unable to read file contents for {}: {reason}",
            readme.display()
        ),
    }

    let first_line = contents.lines().next().unwrap();

    println!("first line of {} is:\n{first_line:?}", readme.display());

    println!()
}

fn file_create() {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let path = Path::new("./target/create.txt");
    let displayable_path = path.display();
    let mut file = match File::create(path) {
        Err(reason) => panic!("Unable to create file for {displayable_path}: {reason}"),
        Ok(result) => result,
    };
    let contents = "foo";

    match file.write(contents.as_bytes()) {
        Err(reason) => panic!("Unable to write to file {displayable_path}: {reason}"),
        Ok(_) => println!("wrote to file!"),
    }

    println!();
}

// `main` is the main thread...
fn main() {
    // threads
    thread_example();
    thread_map_reduce();

    // channels
    channel_example();

    // path
    path_example();

    // file
    file_auto_close();
    file_read();
    file_create();
}
