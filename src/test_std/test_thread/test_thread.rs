use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

#[test]
fn test_arc_mutex() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}

#[test]
fn test_unsafe_seq() {
    let child = thread::spawn(move || {
        for _ in 0..10 {
            unsafe_seq();
            unsafe {
                println!("child = {:?}", V);
            }
        }
    });
    for _ in 0..10 {
        unsafe_seq();
        unsafe {
            println!("test = {:?}", V);
        }
    }
    child.join().unwrap();
}

static mut V: i32 = 0;

fn unsafe_seq() -> i32 {
    unsafe {
        V += 1;
        thread::sleep(Duration::from_millis(1));
        V
    }
}
