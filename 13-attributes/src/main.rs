// will be compile into binary / library only on MacOS
#[cfg(target_os = "macos")]
fn which_os() {
    println!("You're on a mac!")
}

// will be compile into binary / library only on Linux
#[cfg(target_os = "linux")]
fn which_os() {
    println!("You're on linux!")
}

fn main() {
    which_os();

    // cfg! only ever evaluates to true or false, and does so at runtime
    if cfg!(target_os = "macos") {
        println!("You're on a mac!")
    } else if cfg!(target_os = "linux") {
        println!("You're on linux!")
    }
}
