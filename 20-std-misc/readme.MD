# `std` misc

- https://doc.rust-lang.org/stable/rust-by-example/std_misc.html

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
      let result = x.join();

      println!("{result:?}");
  }
  ```

- the OS decides when to execute each thread
