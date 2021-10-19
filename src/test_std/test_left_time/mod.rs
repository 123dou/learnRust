#[test]
fn vec_string_to_str() {
    let mut strings = vec![vec!["123".to_owned(), "456".to_owned(), "789".to_owned()]];
    // let mut strs = vec![];
    // for x in strings.iter() {
    //     strs.push(x.as_str());
    // }
    // println!("strs = {:?}", strs);
    let strs: Vec<Vec<&str>> = strings
        .iter()
        .map(|x| x.iter().map(|x| x.as_str()).collect::<Vec<&str>>())
        .collect();
    println!("strs = {:?}", strs);
}
