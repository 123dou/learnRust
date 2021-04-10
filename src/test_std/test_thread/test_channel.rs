use std::sync::mpsc::{channel, sync_channel};
use std::thread;

#[test]
fn test_channel() {
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    for _ in 0..10 {
        let recv = rx.recv().unwrap();
        println!("recv = {:?}", recv);
    }
}

#[test]
fn test_sync_channel() {
    let (tx, rx) = sync_channel(1);
    tx.send(1).unwrap();
    thread::spawn(move || {
        tx.send(2).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);
}
