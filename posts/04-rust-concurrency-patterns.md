## The Promise of Fearless Concurrency

Concurrent software has historically been fraught with peril: race conditions, deadlocks, and corrupted shared memory.

Rust tackles this foundational problem through its **Ownership & Borrowing** system.

---

### Compile-Time Invariants

* **Exclusive Mutable Access (`&mut T`)**: If you have a mutable reference to data, no other reference can exist simultaneously.
* **Shared Immutable Access (`&T`)**: Any number of threads may read data in parallel so long as no thread is modifying it.
* **Send & Sync Traits**: Types are automatically labeled as safe to transfer across thread boundaries (`Send`) or share between threads (`Sync`).

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Observations

Concurrency bugs become compile-time errors rather than 3:00 AM production outages.
