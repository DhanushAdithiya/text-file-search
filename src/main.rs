use std::collections::HashMap;

mod parser;
mod process_query;
mod text_processing;

fn main() {
    let data = parser::scan_folder("test_folder").unwrap_or_else(|e| {
        eprintln!("Error{}", e);
        HashMap::new()
    });
    let pros = text_processing::tf_idf(&data);
    let query = String::from("what do elephants drink?");
    let pros_query = text_processing::query_if(query);
    println!("SITE: {:?}", process_query::process_query(pros_query, pros));
}
