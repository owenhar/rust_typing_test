use std::fs;

use rand::Rng;

#[cfg(debug_assertions)]
const PATH: &str = "./words.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "/etc/words.txt";

pub fn get_words(number: u32) -> String {
    let mut result = String::new();
    let words = get_words_from_file();

    let mut rng = rand::thread_rng();

    for _ in 0..number {
        let rand_num = rng.gen_range(0..words.len());
        if !result.is_empty() {
            result.push(' ');
        }
        result = result + words.get(rand_num).expect("Failed to index into words")
    }

    return result;
}


fn get_words_from_file() -> Vec<String> {
    let words = fs::read_to_string(PATH).expect("No words found");
    let word_vec = words.split("\n").map(|w| w.to_string()).collect();
    return word_vec;
}