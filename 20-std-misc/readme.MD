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

- threads are scheduled by the OS

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
