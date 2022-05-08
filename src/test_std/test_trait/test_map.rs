use std::collections::HashMap;

#[test]
fn test_entry_or_insert() {
    let mut map = HashMap::new();
    map.insert("1", 1);
    let x = map.entry("1").or_insert(3);
    println!("x = {:?}", x);
    println!("map = {:?}", map);
    let x = map.entry("2").or_insert(3);
    println!("x = {:?}", x);
    println!("map = {:?}", map);
}

#[test]
fn test_entry_insert() {
    let mut map = HashMap::new();
    map.insert("1", 1);
    let entry = map.entry("2").or_insert(2);
    println!("entry= {:?}", entry);
    println!("map = {:?}", map);

    let entry = map.entry("1").or_insert(2);
    println!("entry= {:?}", entry);
    println!("map = {:?}", map);
}
