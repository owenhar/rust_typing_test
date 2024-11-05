use core::f64;
use std::time::SystemTime;
use std::{io::Write, thread, time};
use inline_colorization::*;
use std::io::{self, Read};
use termion::raw::IntoRawMode;
use termion::cursor;


fn main() {
    let testMessage: String = String::from("goat jump cheese eight ball nine ten");
    let mut input: String = String::new();    
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = stdin.lock();

    let mut start = SystemTime::now();
    let mut hasTyped = false;

    write!(stdout, "{}", cursor::Hide).unwrap();
    printLine(&testMessage, &input, false);
    let _ = stdout.flush();
    for c in stdin.bytes() {
        let c = c.unwrap();
        if !hasTyped {
            hasTyped = true;
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
        printLine(&testMessage, &input, true);
        let _ = stdout.flush();

        if testMessage.len() == input.len() {
            break;
        }
    }

    write!(stdout, "{}", cursor::Show).unwrap();
    let _ = stdout.flush();
    drop(stdout);
    println!("");
    println!("Time Elasped: {:.2}", start.elapsed().unwrap().as_secs_f64());
    println!("WPM: {:.2}", (input.len() as f64 / 5.0) / start.elapsed().unwrap().as_secs_f64() * 60.0);
    println!("Accuracy: {:.2}%", accuracy(&testMessage, &input) * 100.0);
}


fn printLine(test : &str, input: &String, overwriteLine: bool) {
    if overwriteLine {
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
        let inChar : char = input.chars().nth(c.0).unwrap();
        if inChar == c.1 {
            print!("{color_blue}{}{color_reset}", c.1);
        } else {
            print!{"{color_red}{}{color_reset}", c.1};
        }
    }
}

fn accuracy(test: &String, input : &String) -> f64 {
    let mut accuracte = 0;
    for c in test.chars().enumerate() {
        if c.1 == input.chars().nth(c.0).unwrap() {
            accuracte += 1;
        }
    }
    return accuracte as f64 / input.len() as f64;
}
