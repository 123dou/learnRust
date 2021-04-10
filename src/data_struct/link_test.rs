use crate::data_struct::link::Link;

#[test]
pub fn test_links() {
    let mut link_list: Link<i32> = Link::new();
    link_list.push(1);
    link_list.push(2);
    link_list.push(3);
    println!("{}", link_list.pop().unwrap());
    println!("{}", link_list.pop().unwrap());
    println!("{}", link_list.pop().unwrap());
}

#[test]
pub fn test_links_iter() {
    let mut link_list: Link<i32> = Link::new();
    link_list.push(1);
    link_list.push(2);
    link_list.push(3);
    for val in link_list.into_iter() {
        println!("{}", val);
    }
}
