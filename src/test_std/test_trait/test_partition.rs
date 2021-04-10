use rand::prelude::SliceRandom;
use rand::thread_rng;

#[test]
fn test_partition() {
    let mut arr = (1..).take(20).collect::<Vec<u32>>();
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);
    let slic = &arr[..];
    println!("slic = {:?}", slic);
    let (p1, p2): (Vec<u32>, Vec<u32>) = slic.iter().partition(|&&x| x >= *arr.last().unwrap());
    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);
}
