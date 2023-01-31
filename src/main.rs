use std::io;

fn main() {
    println!("Please write a string to explode it");

    let mut sentence:String = String::new();
    let mut separator:String = String::new();
    io::stdin().read_line(&mut sentence).expect("Err reading your string");
    let sentence = sentence.trim();

    println!("Please define a separator ?!");
    io::stdin().read_line(&mut separator).expect("Err reading the separator");
    let separator:char = separator.chars().take(1).last().unwrap();
    explode_string(&sentence, separator);
}

fn explode_string(some_string:&str,separator:char)
{
    let bytes = some_string.as_bytes();
    let mut start_index: usize = some_string.len();
    let mut end_index:usize = some_string.len();
    
    for &byte in bytes.iter().rev() {
        
        if byte == separator as u8 {
           println!("Word: {}",&some_string[start_index..end_index]);
           end_index = start_index
        }
        start_index -=1;
    }
    
    println!("Word: {}",&some_string[start_index..end_index]);
    
}
