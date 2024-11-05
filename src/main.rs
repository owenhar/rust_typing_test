use core::f64;
use std::time::SystemTime;
use std::io::Write;
use inline_colorization::*;
use word_gen::words::get_words;
use std::io::{self, Read};
use termion::raw::IntoRawMode;
use termion::cursor;

mod word_gen;

fn main() {
    // let testMessage: String = String::from("goat jump cheese eight ball nine ten");
    let test_message: String = get_words(15);
    let mut input: String = String::new();    
    
    // Sets terminal to raw_mode to stop buffering 
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = stdin.lock();

    // Gets time
    let mut start = SystemTime::now();
    let mut has_typed = false;

    write!(stdout, "{}", cursor::Hide).unwrap(); // Hide the cursor
    print_line(&test_message, &input, false); // Print the text initially
    stdout.flush().unwrap();

    for c in stdin.bytes() {
        let c = c.unwrap();
        if !has_typed {
            has_typed = true;
            start = SystemTime::now()
        }
        if c == 8 || c == 127 {
            if !input.is_empty() {
                input.pop();
            }
        } else if c == 3 {
            break;
        } else {
            input.push(c as char);
        }
        print_line(&test_message, &input, true);
        stdout.flush().unwrap();

        if test_message.len() == input.len() {
            break;
        }
    }

    write!(stdout, "{}", cursor::Show).unwrap(); // Un-hides cursor
    stdout.flush().unwrap();
    drop(stdout); // Stops raw mode and re-enables buffering
    println!("");
    println!("Time Elasped: {:.2}", start.elapsed().unwrap().as_secs_f64());
    println!("WPM: {:.2}", (input.len() as f64 / 5.0) / start.elapsed().unwrap().as_secs_f64() * 60.0);
    println!("Accuracy: {:.2}%", accuracy(&test_message, &input) * 100.0);
}


fn print_line(test : &str, input: &String, overwrite_line: bool) {
    if overwrite_line {
        print!("\r");
    }

    for c in test.chars().enumerate() {
        if c.0 >= input.len() {
            if c.0 == input.len() {
                print!("|");
            }
            print!("{}", c.1);
            continue;
        }
        let in_char : char = input.chars().nth(c.0).unwrap();
        if in_char == c.1 {
            print!("{color_blue}{}{color_reset}", c.1);
        } else {
            print!{"{color_red}{}{color_reset}", c.1};
        }
    }
}

fn accuracy(test: &String, input : &String) -> f64 {
    let mut accuracte = 0;
    for c in input.chars().enumerate() {
        if c.1 == test.chars().nth(c.0).unwrap() {
            accuracte += 1;
        }
    }
    return accuracte as f64 / input.len() as f64;
}
