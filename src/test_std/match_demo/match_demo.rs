use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::Value;

pub fn read_json<P: AsRef<Path>>(fp: P) -> Result<Value, Box<dyn Error>> {
    let fh = File::open(fp)?;
    let reader = BufReader::new(fh);
    let v: Value = serde_json::from_reader(reader)?;
    Ok(v)
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Regular,
    Irregular,
    Outlandish,
}

fn cat_object(v: &Value) -> Category {
    if let Some("black") | Some("white") = v["borderColor"].as_str() {
        Category::Regular
    } else {
        Category::Irregular
    }
}

fn cat_set(v: &str) -> Category {
    match v {
        "SLD" => Category::Outlandish,
        _ => Category::Regular,
    }
}

fn get_data<P: AsRef<Path>>(all_printings: P) -> HashMap<String, Vec<Value>> {
    let printings = read_json(all_printings).unwrap();
    let mut retval = HashMap::new();
    for (set, set_data) in o(&printings["data"]) {
        retval.insert(set.clone(), a(&set_data["cards"]).clone());
    }
    retval
}

pub fn all_printings_to_unique_items(
    data: HashMap<String, Vec<Value>>,
) -> HashMap<Category, HashMap<String, Vec<String>>> {
    let mut retval = HashMap::new();
    for (set, objects) in data {
        for obj in objects {
            if let (Some(uuid), Some(name)) = (obj["uuid"].as_str(), obj["name"].as_str()) {
                retval
                    .entry(max(cat_set(set.as_str()), cat_object(&obj)))
                    .or_insert(HashMap::new())
                    .entry(name.to_string())
                    .or_insert(Vec::new())
                    .push(uuid.to_string())
            }
        }
    }
    retval
}

pub fn o(x: &Value) -> &serde_json::Map<String, Value> {
    x.as_object().unwrap()
}

pub fn a(x: &Value) -> &Vec<Value> {
    x.as_array().unwrap()
}

#[test]
pub fn test_mathc() {
    let mtgjson_json_path = r"src/test_std/match_demo/AllPrintings.json";
    let retval = all_printings_to_unique_items(get_data(mtgjson_json_path));
    println!("{:?}", retval)
}
