use crate::index::{update_index, Comic};

pub async fn get_results(search: String) -> Vec<Comic> {
    // get index
    let index = update_index().await;

    // tokenize search string for words separated by spaces
    let words: Vec<&str> = search.split_whitespace().collect();

    // search index
    let mut results: Vec<Comic> = Vec::new();
    for comic in index {
        for word in &words {
            if comic.blob().to_lowercase().contains(&word.to_lowercase()) {
                results.push(comic.clone());
                break;
            }
        }
    }

    results
}
