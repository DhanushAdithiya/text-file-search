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

    clean.to_ascii_lowercase()
}

fn calculate_tf(
    data: &HashMap<PathBuf, String>,
    n_words: usize,
    words: HashSet<String>,
) -> HashMap<PathBuf, HashMap<String, f64>> {
    let mut tf_values: HashMap<PathBuf, HashMap<String, f64>> = HashMap::new();

    for (url, text) in data {
        let mut temp_hash: HashMap<String, f64> = HashMap::new();

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
    corpus: &HashMap<PathBuf, String>,
    n_docs: usize,
    unique_words: HashSet<String>,
) -> HashMap<String, f64> {
    let mut idf: HashMap<String, f64> = HashMap::new();

    for unique in &unique_words {
        let mut word_count = 0.0;
        for (_, word) in corpus {
            for w in word.split_whitespace() {
                if unique == w {
                    word_count += 1.0;
                }
            }
        }

        let idf_value = (n_docs as f64 / word_count).ln();
        idf.insert(unique.clone(), idf_value);
    }
    // println!("IDF VALUES: \n{idf:#?}");
    idf
}

//calculates tf_idf

pub fn tf_idf(data: &HashMap<PathBuf, String>) -> HashMap<PathBuf, HashMap<String, f64>> {
    let mut unique_words: HashSet<String> = HashSet::new();
    let mut tf_idf: HashMap<PathBuf, HashMap<String, f64>> = HashMap::new();

    // Step 1: Calculate unique words across all documents
    for document in data.values() {
        for word in document.split_whitespace() {
            unique_words.insert(word.to_string());
        }
    }

    let n_words = unique_words.len();
    let n_docs = data.len();

    // Step 2: Calculate TF and IDF values
    let tf_values = calculate_tf(data, n_words, unique_words.clone());
    let idf_values = calculate_idf(data, n_docs, unique_words.clone());

    // Step 3: Calculate TF-IDF scores
    for (path, words) in tf_values.into_iter() {
        let mut temp_hash: HashMap<String, f64> = HashMap::new();
        for (word, tf_value) in words.into_iter() {
            if let Some(val) = idf_values.get(&word) {
                let tf_idf_value = val * tf_value;
                temp_hash.insert(word, tf_idf_value);
            }
        }
        tf_idf.insert(path, temp_hash);
    }

    // println!("{:#?}", tf_idf);
    tf_idf
}
