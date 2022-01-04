use std::io; 

/* 
Task: Convert strings to pig latin. 
The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */
fn main() {
    pig_latin();
}

fn pig_latin() {

    println!("Type any amount of words and hit enter!"); 

    let mut input_text = String::new(); 

    io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin"); 

    // Save string in Vector divided by whitespaces
    let words: Vec<&str> = input_text.split_whitespace().collect(); 
    let mut pig_latin = String::new(); 

    // Concat new text 
    for word in &words {
       pig_latin.push_str(&pigify_word(word));
       pig_latin.push(' '); 
    }
    println!("Original: {}", input_text);
    println!("Pig latin: {}", pig_latin); 
}

fn pigify_word(word :&str) -> String {
    let first_char = word.chars().next().unwrap(); 
    let last_char = word.chars().next_back().unwrap(); //Check for any non-alphanumeric signs like '.' ',' etc 

    // Ignore char-combinations which are not words
    if !first_char.is_alphabetic(){
        String::from(word)
    }
    else {
        if last_char.is_alphanumeric()
        {
            match first_char {
                'a' | 'e' | 'i' | 'o' | 'u'| 'A' | 'E' | 'I' | 'O' | 'U'  => format!("{}-hay", word),
                _ => format!("{}-{}ay", &word[1..], first_char)
            }
        }
        else {
            match first_char {
                'a' | 'e' | 'i' | 'o' | 'u'| 'A' | 'E' | 'I' | 'O' | 'U'  => format!("{}-hay{}", &word[..word.len()-1], last_char),
                _ => format!("{}-{}ay{}", &word[1..word.len()-1], first_char, last_char)
            }
        }
    }
}