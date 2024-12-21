use std::thread;
use std::time::Duration;

fn main() {
    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Spawn 5 threads
    for i in 1..=5 {
        let handle = thread::spawn(move || {
            println!("Thread {} is starting.", i);
            thread::sleep(Duration::from_secs(2));
            println!("Thread {} is done.", i);
        });

        // Push the handle into the vector
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have completed.");
}
