use std::collections::HashMap;
use std::io::{Read, Write};
use std::str::FromStr;

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (key, val) in self.map {
            let record = format!("{}\t{}\n", key, val);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }

    pub fn save_json(self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .truncate(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(file, &self.map)?;
        Ok(())
    }

    pub fn complete(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key) {
            Some(val) => {
                *val = false;
                Some(())
            }
            None => None,
        }
    }
}

pub(crate) fn new() -> Result<Todo, std::io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt")?;
    let mut ctxt = String::new();
    file.read_to_string(&mut ctxt)?;
    let map = ctxt
        .lines()
        .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        .map(|ve| (ve[0], ve[1]))
        .map(|(key, val)| (String::from(key), bool::from_str(val).unwrap()))
        .collect::<HashMap<String, bool>>();
    Ok(Todo { map })
}

pub(crate) fn new_json() -> Result<Todo, std::io::Error> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.json")?;
    match serde_json::from_reader(file) {
        Ok(map) => Ok(Todo { map }),
        Err(e) if e.is_eof() => Ok(Todo {
            map: HashMap::new(),
        }),
        Err(err) => panic!("serde json file fail. {}", err),
    }
}
