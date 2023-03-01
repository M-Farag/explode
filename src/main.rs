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
    let words = explode_string(&sentence, separator);
    println!("All words: {:#?}",words);
}

fn explode_string(some_string:&str,separator:char) -> Vec<&str>
{
    let mut start_index:usize = 0;
    let mut words: Vec<&str> = Vec::new();
    
    for (end_index, letter) in some_string.chars().enumerate() {
        
        if letter == separator {
           words.push(&some_string[start_index..end_index]);
           start_index = end_index +1
        }
    }
    words.push(&some_string[start_index..]);
    words
    
}
