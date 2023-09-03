use std::{collections::HashMap, path::PathBuf};

pub fn process_query(
    query: HashMap<String, f64>,
    data: HashMap<PathBuf, HashMap<String, f64>>,
) -> PathBuf {
    let mut res: PathBuf = PathBuf::new();
    let mut query_vec: Vec<f64> = Vec::new();
    let mut cosine_score = 0.0;
    for (_word, score) in &query {
        query_vec.push(*score);
    }

    for (url, doc_data) in &data {
        let mut document_vec: Vec<f64> = Vec::new();
        for (q_word, _q_scre) in &query {
            if let Some(score) = doc_data.get(q_word) {
                document_vec.push(*score);
            }
        }

        if !document_vec.is_empty() {
            let cos_sim = cosine_similarity(&query_vec, &document_vec);
            if cos_sim > cosine_score {
                cosine_score = cos_sim;
                res = url.clone();
            }
        }
    }

    res
}

fn dot_prod(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum()
}

fn mag(vector: &Vec<f64>) -> f64 {
    vector.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn cosine_similarity(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let dot = dot_prod(v1, v2);
    let mag1 = mag(v1);
    let mag2 = mag(v2);
    if mag1 > 0.0 && mag2 > 0.0 {
        dot / (mag1 * mag2)
    } else {
        0.0
    }
}
