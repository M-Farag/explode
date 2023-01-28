use std::io;

fn main() {
    println!("Please write a string to explode it");

    let mut sentence:String = String::new();
    io::stdin().read_line(&mut sentence).expect("Err reading your string");
    let sentence = sentence.trim();
    explode_string(&sentence);
}

fn explode_string(some_string:&str)
{
    let bytes = some_string.as_bytes();
    let mut start_index: usize = some_string.len();
    let mut end_index:usize = some_string.len();

    for &byte in bytes.iter().rev() {
        if byte == b' ' {
           println!("Word: {}",&some_string[start_index..end_index]);
           end_index = start_index
        }
        start_index -=1;
    }
    
    println!("Word: {}",&some_string[start_index..end_index]);
    
}
