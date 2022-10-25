use std::io; 
use std::collections::HashMap; 

/*  Task: 
    Given a list of integers, use a vector and return the mean (the average value), 
    median (when sorted, the value in the middle position), 
    and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    Using knowledge acquired in the chapters 1-8 (Except in get_average where I used a iter which will be introduced in Chp.13)
 */

fn main() {
   numbers_input();
}

fn numbers_input() {

    // INPUT
    println!("Type any amount of numbers. To stop enter anything that is not an integer!"); 
    let mut v: Vec<u32> = Vec::new(); 
    loop {
        let mut input_text = String::new(); 

        io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin"); 
        
        let trimmed = input_text.trim(); 
        match trimmed.parse::<u32>() {
            Ok(i) => v.push(i),
            Err(_) => break,
        }
    }

    // Average
    let average = get_average( &v);
    println!("The average is: {}", average);

    // Median
    let median = get_median(&mut v);
    println!("The median is: {}", median);
    

    //Mode
    let (mode, most) = get_mode(&v); 
    println!("The mode is: {} which occurs {} times", mode, most); 
}

fn get_average(input_vector :&Vec<u32>) -> f32 {
    input_vector.iter().sum::<u32>() as f32 / input_vector.len() as f32
}

fn get_median(input_vector :&mut Vec<u32>) -> f32 {
    let vector_length = input_vector.len(); 
    input_vector.sort(); 

    if vector_length % 2 == 0 
    {
        (input_vector[vector_length/2 - 1] as f32 + input_vector[vector_length / 2] as f32) / 2.0
    }
    else {
        input_vector[vector_length/2] as f32
    }
}

// Returns a tuple because I wanted to have the most used value and how often it's being used
// Not sure if I should create a struct here and return a struct instead
fn get_mode(input_vector :&Vec<u32>) -> (u32, i32) {
    let mut map = HashMap::new(); 

    for val in input_vector {
        let count = map.entry(val).or_insert(0);
        *count += 1; 
    }

    let mut mode: u32 = 0; 
    let mut most: i32 = 0; 
    // Iterate through vector to find key with/and highest value
    for key_value_pairs in map {
        if key_value_pairs.1 > most {
            mode = *key_value_pairs.0;
            most = key_value_pairs.1; 
        }
    }

    (mode, most)
}
