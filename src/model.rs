use std::{collections::HashMap, io::Read};
use std::str::FromStr;
const DATABASE_FILE: &str = "db.txt";

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(DATABASE_FILE)?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    pub fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key){
            Some(v) => Some(*v = true),
            None => None
        }
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write(DATABASE_FILE, content)
    }
}
