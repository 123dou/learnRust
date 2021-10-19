// 给定一个字符串或者数字，比如 42
// 给定一个工作目标：找到另外一个数字，要求该数字和42相乘后的结果，经过Hash函数处理后，满足得到的加密串以“00000”开头。可以通过堆“00000”增加或者减少0的个数来控制查找的难度
// 为来找到该数字需要从1开始递增查找，直到找到满足条件的数字。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};

use crypto::digest::Digest;
use crypto::sha2::Sha256;

const BASE: usize = 42;
const THREADS: usize = 12;
const DIFFICULTY: &'static str = "12345";

#[test]
fn proof_of_work() {
    println!(
        "PoW: Find a number SHA256(number * {}) == \"{}...\"",
        BASE, DIFFICULTY
    );
    println!("start threads = {:?}", THREADS);
    let start = std::time::Instant::now();
    let is_solution = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel();
    for i in 0..THREADS {
        let tx = tx.clone();
        let is_solution = is_solution.clone();
        std::thread::spawn(move || {
            find_solution(i, tx, is_solution);
        });
    }
    match rx.recv() {
        Ok(Solution(num, hash)) => {
            println!("found solution: cost time: {}s", start.elapsed().as_secs());
            println!("the number is {}, and the hash is {}", num, hash);
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}

struct Solution(usize, String); // usize为找到的数字，String为密文

fn verify(num: usize) -> Option<Solution> {
    let res = num * BASE;
    let mut hasher_256 = Sha256::new();
    hasher_256.input_str(res.to_string().as_str());
    let hash = hasher_256.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(res, hash))
    } else {
        None
    }
}

fn find_solution(start_at: usize, tx: Sender<Solution>, is_solution: Arc<AtomicBool>) {
    for i in (start_at..).step_by(THREADS) {
        if is_solution.load(Ordering::Relaxed) {
            return;
        }
        if let Some(solution) = verify(i) {
            is_solution.store(true, Ordering::Relaxed);
            tx.send(solution);
            return;
        }
    }
}
