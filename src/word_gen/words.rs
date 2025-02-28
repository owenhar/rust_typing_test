use std::fs;

use rand::Rng;

#[cfg(debug_assertions)]
const PATH: &str = "./words.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "/etc/words.txt";

pub fn get_words(number: usize) -> Vec<String> {
    let words = get_words_from_file();

    let mut rng = rand::thread_rng();

    let mut final_words: Vec<String> = Vec::with_capacity(number as usize);
    for _ in 0..number {
        let rand_num = rng.gen_range(0..words.len());
        final_words.push(
            words
                .get(rand_num)
                .expect("Failed to index into words")
                .clone(),
        );
    }

    return final_words;
}

fn get_words_from_file() -> Vec<String> {
    let words = fs::read_to_string(PATH).expect("No words found");
    let word_vec = words.split("\n").map(|w| w.to_string()).collect();
    return word_vec;
}
