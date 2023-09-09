use crate::parser;
use crate::text_processing;
use std::{collections::HashMap, fs::File, io::Write};

pub fn index_folder(path: &str) {
    let data = parser::scan_folder(path).unwrap_or_else(|e| {
        eprintln!("Error{}", e);
        HashMap::new()
    });
    let pros = text_processing::tf_idf(&data);

    let serialized = serde_json::to_string(&pros).unwrap();
    let mut file = File::create("data.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap()
}

pub fn search_query() {}
