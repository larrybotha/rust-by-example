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
    use std::path::Path;

    let path = Path::new(".");

    println!("displayable path: {}\n", path.display());

    for child in path.read_dir().unwrap() {
        println!("child: {child:?}");
    }

    let new_path = path.join("foo").join("bar");

    [new_path]
        .into_iter()
        .inspect(|path| println!("new_path: {}", path.display()))
        .map(|mut path| {
            path.push("baz");
            path
        })
        .inspect(|path| println!("new_path: {}", path.display()))
        .map(|mut path| {
            path.push("my-awesome-filename.txt");
            path
        })
        .inspect(|path| println!("new_path: {}", path.display()))
        .map(|mut path| {
            path.set_file_name("change-that-filename.txt");
            path
        })
        .for_each(|path| println!("new_path: {}", path.display()));

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
    use std::fs;
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

    match fs::remove_file(path) {
        Err(reason) => panic!("Unable to remove file: {displayable_path}: {reason}"),
        Ok(_) => println!("removed file: {displayable_path}"),
    }

    println!();
}

fn create_file(path: &std::path::Path, contents: String) -> std::fs::File {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let displayable_path = path.display();

    let mut file = match File::create(Path::new(path)) {
        Err(reason) => panic!("Unable to create file {displayable_path}: {reason}"),
        Ok(result) => result,
    };

    if let Err(reason) = file.write(contents.as_bytes()) {
        panic!("Unable to write to file {displayable_path}: {reason}")
    };

    file
}

fn read_lines_beginner() {
    use std::fs::{self, File};
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;

    // This is the beginner implementation here
    //
    // It is less efficient, because we render the entire string into memory
    // before returning it.
    // Instead, we could allow for each line to be read one at a time out of
    // the file, as in the read_lines_efficient function
    fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
        // open the file in read-only mode
        let file = File::open(filename).unwrap();

        // read the file:
        //  - line by line
        //  - returning an iterator of the lines
        io::BufReader::new(file).lines()
    }

    let path = Path::new("./target/read-lines-beginner.txt");
    let contents = ["foo", "bar", "baz"].join("\n");

    create_file(path, contents);

    // From ChatGPT:
    //  Storing the iterator may result in holding unnecessary references and
    //  additional memory usage, especially if the iterator is large or the
    //  loop executes for a long time.
    let lines = read_lines(path.to_str().unwrap().into());

    lines
        .into_iter()
        .map(|line| line.unwrap())
        .enumerate()
        .for_each(|(index, line)| println!("line {}: {line}", index + 1));

    if let Err(reason) = fs::remove_file(path) {
        panic!("Unable to remove file {}: {reason}", path.display())
    }

    println!()
}

fn read_lines_efficient() {
    use std::fs::{self, File};
    use std::io::{self, BufRead};
    use std::path::Path;

    // Return a result, instead of the iterator.
    fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;

        Ok(io::BufReader::new(file).lines())
    }

    let path = Path::new("./target/read-lines-efficient.txt");
    let content = ["foo", "bar", "baz"].join("\n");

    create_file(path, content);

    // It's more efficient _not_ to assign the iterator to a variable.
    // Instead, we consume it immediately, without holding onto a
    // potentially large amount of data
    if let Ok(lines) = read_lines(path) {
        for (index, line) in lines.flatten().enumerate() {
            println!("line {}: {line}", index + 1)
        }
    };

    if let Err(reason) = fs::remove_file(path) {
        panic!("Unable to remove file {}: {reason}", path.display())
    }

    println!()
}

fn child_process_example() {
    use std::process::Command;

    // Command uses the builder pattern, where '.output' results in the actual
    // execution of the child process
    let output = Command::new("rustc")
        .args(["--version"])
        .output()
        .unwrap_or_else(|e| panic!("Unable to run child process: {}", e));

    [output]
        .iter()
        .map(|x| match x.status.success() {
            true => Ok(String::from_utf8_lossy(&x.stdout)),
            false => Err(String::from_utf8_lossy(&x.stderr)),
        })
        .for_each(|result| match result {
            Err(err) => println!("Child process errored: {err}"),
            Ok(out) => println!("Child process succeeded: \n\t{out}"),
        });

    println!()
}

fn child_process_pipes() {
    use std::io::{Read, Write};
    use std::process::{Child, Command, Stdio};

    const PANGRAM: &str = "the quick brown fox jumped over the lazy dog";

    // redundant type here for clarity
    let process: Child = match Command::new("wc")
        // allow for the child to be piped to
        .stdin(Stdio::piped())
        // allow for the child to be read from via pipes
        .stdout(Stdio::piped())
        // spawn the process
        .spawn()
    {
        Ok(process) => process,
        Err(reason) => panic!("Unable to spawn child process: {reason}"),
    };

    // write bytes to the child's stdin
    //
    // we can use .unwrap() here because _we_ specified that the command
    // can be piped to - i.e. in this particular case we know that it's
    // safe to unwrap without handling errors
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("Unable to pipe to wc: {}", why),
        Ok(_) => println!("Sent pangram to wc"),
    }

    let mut output = String::new();

    // read from the child's stdout
    //
    // the same applies for unwrapping here - we know that stdout can be
    // safely unwrapped, because we specified that we will be communicating
    // via pipes when the child was built
    match process.stdout.unwrap().read_to_string(&mut output) {
        Err(why) => panic!("Unable to read output from wc: {}", why),
        Ok(_) => println!("output from wc read"),
    }

    println!("wc output: {}", output);
    println!();
}

fn child_process_wait() {
    use std::process::Command;

    let sleepy_time = 1;
    let mut child = Command::new("sleep")
        .arg(format!("{}", sleepy_time))
        .spawn()
        .unwrap();

    println!("sleeping {}s...", sleepy_time);

    // explicitly wait for the child process to complete
    let exit_status = child.wait().unwrap();

    println!("{:#?}", exit_status);
    println!();
}

fn filesystem_operations() {
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::{Read, Write};
    use std::path::Path;

    fn cat(path: &Path) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(e) => Err(e),
        }
    }

    // similar to `$ echo foo > my-file.txt`
    fn echo_to_file(value: &str, path: &Path) -> io::Result<()> {
        let mut file = File::open(path)?;

        file.write_all(value.as_bytes())
    }

    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
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

    // read_lines
    read_lines_beginner();
    read_lines_efficient();

    // child processes
    child_process_example();
    child_process_pipes();
    child_process_wait();

    // filesystem operations
    filesystem_operations();
}
