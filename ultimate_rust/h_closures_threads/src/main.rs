use std::thread;
use std::time::Duration;

use crossbeam::channel;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let handle = thread::spawn(move || {
        expensive_sum(my_vector)
    });
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }
    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);

    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(200);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100);

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(0);
        tx.send("Thread B: 2").unwrap();
    });


    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    let (tx, rx) = channel::unbounded::<String>();
    let rx1 = rx.clone();
    for i in 1..5 {
        let message = format!("Thread C: {}", i);
        println!("Main thread: Sending {}", message);
        tx.send(message).unwrap();
    }
    let child1 = thread::spawn(move || {
        for msg in rx {
            pause_ms(50);
            println!("Child thread 1: Received {}", msg);
        }
    });
    let child2 = thread::spawn(move || {
        for msg in rx1 {
            pause_ms(50);
            println!("Child thread 2: Received {}", msg);
        }
    });
    drop(tx);
    child1.join().unwrap();
    child2.join().unwrap();
    println!("Main thread: Exiting.")
}
