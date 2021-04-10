use std::io::Error;

mod to_do;

fn main() {
    let action = std::env::args().nth(1).expect("need a action");
    let item = std::env::args().nth(2).expect("need a item");
    let save_type = std::env::args().nth(3).expect("must be txt or json");
    if save_type == "txt" {
        save_txt(action, item);
    } else if save_type == "json" {
        save_json(action, item);
    }
}

fn save_txt(action: String, item: String) {
    let mut todo = to_do::new().expect("initial db fail");
    if action == "add" {
        todo.insert(item.clone());
        match todo.save() {
            Ok(_) => {
                println!("add a to do item={} pass", item.clone());
            }
            Err(err) => {
                println!("add item={} has a error={}.", item.clone(), err);
            }
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            Some(_) => match todo.save() {
                Ok(_) => {
                    println!("set item={} complete pass.", item.clone());
                }
                Err(err) => {
                    println!("add item={} has a error={}.", item.clone(), err);
                }
            },
            None => println!("item {} had not add in lists", item.clone()),
        }
    }
}

fn save_json(action: String, item: String) {
    let mut todo = to_do::new_json().expect("initial db fail");
    if action == "add" {
        todo.insert(item.clone());
        match todo.save_json() {
            Ok(_) => {
                println!("add a to do item={} pass", item.clone());
            }
            Err(err) => {
                println!("add item={} has a error={}.", item.clone(), err);
            }
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            Some(_) => match todo.save_json() {
                Ok(_) => {
                    println!("set item={} complete pass.", item.clone());
                }
                Err(err) => {
                    println!("add item={} has a error={}.", item.clone(), err);
                }
            },
            None => println!("item {} had not add in lists", item.clone()),
        }
    }
}
