use std::{env, fs::File, io::BufReader};
mod commands;
mod parser;
mod process_query;
mod text_processing;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Useage: cargo run -- index 'location' or cargo run -- query 'word' ");
        return;
    }

    let command = args[1].parse::<String>().unwrap();
    let argument = args[2].as_str();

    match command.as_str() {
        "index" => commands::index_folder(argument),
        "query" => {
            let file = File::open("data.json").expect("You need to index a folder first!");
            let reader = BufReader::new(file);
            let map = serde_json::from_reader(reader).unwrap();
            let pros_query = text_processing::query_if(String::from(argument));
            println!("SITE: {:?}", process_query::process_query(pros_query, map));
        }
        _ => {
            eprintln!("There is something wrong please check use cargo run --help")
        }
    }
}
