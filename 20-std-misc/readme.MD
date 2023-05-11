# `std` misc

- https://doc.rust-lang.org/stable/rust-by-example/std_misc.html
- [mpsc - Multi Producer, Single Consumer](http://doc.rust-lang.org/1.69.0/std/sync/mpsc/index.html)
- http://doc.rust-lang.org/1.69.0/std/path/struct.Path.html

## Takeaways

### Threads

- native OS threads can be spawned using `std::thread`
- threads allow for running multiple tasks on multiple CPUs in the background,
  concurrently, which can greatly assist in improving responsiveness in the UI
- `main` happens to be the main thread the application is running in
- once a thread is created, one can wait for the thread to finish using
  `some_thread.join()`:

  ```rust
  use std::thread;

  let threads = vec![];

  for i in 0..5 {
      let my_thread = thread::spawn(move || {
          println!("pew pew pew {i}");

          format!("here you go: {i}")
      });

      threads.push(my_thread);
  }

  for x in threads {
    // JoinHandle::join is synchronous, and waits for the thread to
    // complete before moving on
    let result = x.join();

    println!("{result:?}");
  }
  ```

- threads are scheduled by the OS
- threads are not an infinite resource - it's a good idea to limit the number of
  concurrent threads that may be spawned

### Channels

- channels allow for communication between threads in a uni-directional manner -
  values can be produced by multiple senders, but can only be received by a
  single receiver
- `mpsc` means Multiple Producer, Single Consumer, which describes how channels
  work:

  - we create a channel that returns a tuple with a sender and receiver:

    ```rust
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    ```

  - we send messages using the sender:

    ```rust
    use std::thread;

    thread::spawn(move || {
        tx.send("some value").unwrap();
    })
    ```

  - we receive messages with the receiver:

    ```rust
    let result = rx.recv().unwrap()
    ```

### Path

- `Path` and `PathBuf` have a similar relationship to each other as `&str` and
  `String`, where `Path` is a reference, and `PathBuf` is owned
- `Path` values are immutable
- `PathBuf` values are mutable, and can be dereferenced to `Path
- Rust will automatically handle posix and Windows-style paths behind the scenes
- using `Path::join` to create a new path results in a `PathBuf`
- see [Metadata](http://doc.rust-lang.org/1.69.0/std/fs/struct.Metadata.html)
  struct for more information related to files
- `Path::metadata` returns information about a file or directory similar to the
  Unix `stat` command

### File / IO

- `File` represents an opened file, and wraps a file descriptor
- because many things can go wrong when working with files, many of the methods
  on `File` return `io::Result<T>`, which is an alias of `Result<T, io::Error>`
- reading from files requires that `std::io::Read` is present in the
  module - `std::io::Read` extends the `File` struct with the `.read_to_string`
  method
- writing to files requires that `std::io::Write` is used in the module, which
  extends the `File` struct with a `.write` method

#### `open`

- as with context managers in Python, an opened file is automatically closed in
  Rust when the value that owns the file descriptor is dropped
- to open a file, we use the `File` module with `Path`:

  ```rust
  use std::fs::File;
  use std::path::Path;

  let path = Path::new("path/to/file.ext");
  let file = match File::open(path) {
      Ok(result) => result,
      Err(reason) => panic!("unable to open file {}: {reason}", path.display()),
  };
  ```

#### `create`

- `create` will do one of the following when being called:
  - if the file exists, the file's contents will be cleared
  - open a new file
- `create` opens files in a write-only mode
- `File::write` expects a byte string - not a string - to be passed to it to
  write to the file. Additionally, the file needs to be mutable in order to
  write to it:

  ```rust
  use std::io::Write;
  use std::fs::File;
  use std::path::Path;

  let path = Path::new("./to/file.txt");
  let contents = "foo";
  // File must be mutable to write to
  let mut file = match File::create(path) {
      Err(reason) => panic!("unable to create file {}: {reason}", path.display()),
      Ok(result) => result,
  };

  // .write is present because of std::io::Write
  match file.write(contents) {
      Err(reason) => panic!(
        "unable to write to file {}: {reason}",
        path.display()
      );
      Ok(_) => println!("wrote to file!")
  };
  ```

## Additional

- Rust suggests avoiding `Array::map` for large arrays and when
  [chaining `.map`](http://doc.rust-lang.org/1.69.0/std/primitive.array.html#method.map)
  due to performance issues related to how much of the stack becomes
  utilised, and instead converting the array into an iterator using `.iter` or
  `.into_iter`
- `.map` for side effects
  [is not considered idiomatic Rust](http://doc.rust-lang.org/1.69.0/std/iter/trait.Iterator.html#method.map)
  -- `for` loops are suggested for code where there is a side effect in a loop
- `Iterator::inspect` works like `.map`, allowing for something to be done with
  the value in the iteration, but then implicitly returns the iterated value
- the current thread ID can be retrieved as such:

  ```rust
  thread::spawn(|| {
      println!("my thread ID is {:?}", thread::current().id());
  })
  ```

- using `.map` on on iterable returns a `Map` struct - note its difference to
  `HashMap`: http://doc.rust-lang.org/1.69.0/std/iter/struct.Map.html
- `.collect` only cares about what values are being collected _into_ - not what
  the actual values are. e.g. are we collecting into a `Vec` or `VecDeque`?

  As a result, much of the time we can let `.collect` infer the type of the
  value inside the collection:

  ```rust
  let my_string = "hey hey hey!";
  let explicit_vec: Vec<&str> = my_string.split_whitespace().collect();
  let inferred_vec: Vec<_> = my_string.split_whitespace().collect();
  ```

- `.map(...).for_each(drop)` may be unnecessary, as `.for_each` will drop values
  anyways. The docs in both Rust and Javascript indicate that the use-case for
  `.for_each` / `.forEach` is the execution of side effects
- `static` vs `const`:
  - use `static` when:
    - you want a global variable that is mutable
  - use `const` when:
    - you know the value at compile-time, and know that it won't ever change
