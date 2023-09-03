use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn stop_word_removal(input: &String) -> String {
    let stop_words: [&str; 165] = [
        "i",
        "me",
        "my",
        "myself",
        "we",
        "our",
        "ours",
        "ourselves",
        "you",
        "your",
        "yours",
        "yourself",
        "yourselves",
        "he",
        "him",
        "his",
        "himself",
        "she",
        "her",
        "hers",
        "herself",
        "it",
        "its",
        "itself",
        "they",
        "them",
        "their",
        "theirs",
        "themselves",
        "what",
        "which",
        "who",
        "whom",
        "this",
        "that",
        "these",
        "those",
        "am",
        "is",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "having",
        "do",
        "does",
        "did",
        "doing",
        "a",
        "an",
        "the",
        "and",
        "but",
        "if",
        "or",
        "because",
        "as",
        "until",
        "while",
        "of",
        "at",
        "by",
        "for",
        "with",
        "about",
        "against",
        "between",
        "into",
        "through",
        "during",
        "before",
        "after",
        "above",
        "below",
        "to",
        "from",
        "up",
        "down",
        "in",
        "out",
        "on",
        "off",
        "over",
        "under",
        "again",
        "further",
        "then",
        "once",
        "here",
        "there",
        "when",
        "where",
        "why",
        "how",
        "all",
        "any",
        "both",
        "each",
        "few",
        "more",
        "most",
        "other",
        "some",
        "such",
        "no",
        "nor",
        "not",
        "only",
        "own",
        "same",
        "so",
        "than",
        "too",
        "very",
        "s",
        "t",
        "can",
        "will",
        "just",
        "don",
        "should",
        "now",
        "#",
        "##",
        "###",
        "####",
        "#####",
        "######",
        ".",
        ",",
        "?",
        "!",
        ":",
        ";",
        "-",
        "'",
        "\"",
        "(",
        ")",
        "[",
        "]",
        "{",
        "}",
        "<",
        ">",
        "/",
        "\\",
        "|",
        "_",
        "&",
        "#",
        "*",
        "@",
        "%",
        "~",
        "=",
        "+",
        "`",
        "^",
        "$",
    ];
    let mut clean = input.clone();

    for stop_word in stop_words.iter() {
        clean = clean
            .split_whitespace()
            .filter(|word| word != stop_word)
            .collect::<Vec<&str>>()
            .join(" ");
    }

    clean
}

//fn calculate_tf(
//    corpus: &Vec<String>,
//    n_words: usize,
//    words: HashSet<String>,
//) -> HashMap<String, f64> {
//    let mut tf_values: HashMap<String, f64> = HashMap::new();
//    for unique in words {
//        let mut count = 0.0;
//        for doc in corpus {
//            for word in doc.split_whitespace() {
//                if word == unique {
//                    count += 1.0;
//                }
//            }
//        }
//
//        let tf = count / n_words as f64;
//        tf_values.insert(unique, tf);
//    }
//    tf_values
//}

fn calculate_tf(
    data: HashMap<PathBuf, String>,
    n_words: usize,
    words: HashSet<String>,
) -> HashMap<String, HashMap<String, f64>> {
    let mut tf_values: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for (url, text) in &data {
        let mut temp_hash: HashMap<PathBuf, f64> = HashMap::new();

        for unique in &words {
            let word_count = text
                .split_whitespace()
                .filter(|word| word == unique)
                .count() as f64;
            let tf = word_count / n_words as f64;
            temp_hash.insert(unique.clone(), tf);
        }

        tf_values.insert(url.clone(), temp_hash);
    }

    tf_values
}

pub fn query_if(query: String) -> HashMap<String, f64> {
    let length = query.split_whitespace().count();
    let mut unique: HashSet<String> = HashSet::new();
    let mut tf: HashMap<String, f64> = HashMap::new();
    let clean = stop_word_removal(&query.clone());
    for word in clean.split_whitespace() {
        unique.insert(word.to_string());
    }

    for u_words in unique {
        let mut count: f64 = 0.0;
        for word in clean.split_whitespace() {
            if word == u_words {
                count += 1.0;
            }
        }
        let tf_value = count / length as f64;
        tf.insert(u_words, tf_value);
    }

    tf
}

fn calculate_idf(
    corpus: &Vec<String>,
    n_docs: usize,
    words: HashSet<String>,
) -> HashMap<String, f64> {
    let mut idf_values: HashMap<String, f64> = HashMap::new();
    for unique in words {
        let mut count = 0.0;
        for doc in corpus {
            if doc.contains(&unique) {
                count += 1.0;
            }
        }
        let idf = count / n_docs as f64;
        idf_values.insert(unique, idf);
    }
    idf_values
}

pub fn tf_idf(data: HashMap<PathBuf, String>) -> HashMap<String, HashMap<String, f64>> {
    // Recieve the url + the clean content and then process it and return the results
    let mut unique_words: HashSet<String> = HashSet::new();
    //let n_docs = corpus.len();

    for (_url, document) in data.clone() {
        for word in document.split_whitespace() {
            unique_words.insert(String::from(word));
        }
    }
    let n_words = unique_words.len();

    let tf_values = calculate_tf(data, n_words, unique_words.clone());
    //let idf_values = calculate_idf(&corpus, n_docs, unique_words.clone());
    //    let mut tf_idf_values: HashMap<String, f64> = HashMap::new();
    //
    //    for word in tf_values {
    //        let value = idf_values.get(&word.0).unwrap();
    //        tf_idf_values.insert(word.0, value * word.1);
    //    }

    println!("{:#?}", tf_values);
    tf_values
}

//(hello,1.234)
//{
//doc1 ,{
//  hello = 0.1,
//  world = 0.9
//},
//doc2, {
//  hi ' 0.01,
//  seamen = 0.6'
//}
//}
//
