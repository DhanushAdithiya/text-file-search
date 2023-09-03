mod parser;
//mod process_query;
mod text_processing;

fn main() {
    let data = parser::scan_folder("test_folder");
    let pros = text_processing::tf_idf(data);
}
