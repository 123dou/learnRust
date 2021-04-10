#[derive(Debug)]
struct Val(i32);

impl Drop for Val {
    fn drop(&mut self) {
        println!("drop for {:?}", self.0);
        // todo!();
    }
}

#[test]
fn test_shadowing() {
    let mut x = Val(1);
    println!("create x = {:?}", x);
    {
        x = Val(3);
        println!("create temp x  = {:?}", x);
    }
    let x = Val(2);
    println!("create shadowing = {:?}", x);
}
