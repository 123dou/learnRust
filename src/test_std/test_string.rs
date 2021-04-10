use std::collections::VecDeque;

#[test]
fn test_drain() {
    let s = "";
    let _string = "".to_string();
    let deque = s.split(",").collect::<VecDeque<&str>>();
    println!("vec = {:?}", deque);
    for i in 0..deque.len() {
        if "n" != deque[i] {
            println!("i32 = {:?}", deque[i].parse::<i32>().unwrap());
        }
    }
}
