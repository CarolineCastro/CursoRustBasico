use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn pause_ms(ms: u64){
    thread::sleep(Duration::from_millis(ms));
}


fn main() {
    
    let (tx, rx) = channel::unbounded();
    
    let rx2 = rx.clone();

    let handle_a = thread::spawn(move || {
        for msg in rx {
            println!("Thread A received: {}", msg);
        }
    });

    let handle_b = thread::spawn(move || {
        for msg in rx2 {
            println!("Thread B received: {}", msg);
        }
    });


    for i in 0..50 {
        tx.send(i).unwrap();
        pause_ms(20);
    }

    drop(tx);


    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!("Main thread: Exiting");
}
