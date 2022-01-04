use std::io; 

// Converts an input from fahrenheit to celsius or vice versa
fn main() {
   loop{
        let mut input = String::new(); 
        let mut value = String::new(); 
        println!("Shall I convert to Fahrenheit(f) or Celsius(c) ?");

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        // Only continue if input is C/c or F/f
        match input.trim().to_lowercase().as_str() {
            "c" |"f" => {},
            _ => 
            {
                println!("Can not convert to {}", input);
                continue;
            },
        }

        println!("You want to convert to {}", input); 
        println!("Which value do you want to convert?"); 

        io::stdin()
            .read_line(&mut value)
            .expect("Could not read value"); 
        
        let value :f32 = value.trim().parse().expect("Can not convert the value"); 

        match input.trim().to_lowercase().as_str() {
            "c" => println!("{}c", to_celsius(value)),
            "f" => println!("{}f", to_fahrenheit(value)),
            _ => println!("Could not convert value"),
        }
}

}

fn to_fahrenheit(x :f32) -> i32{
    ((x * 1.8) + 32.0) as i32
}

fn to_celsius(x :f32) -> i32{
    ((x - 32.0) * (5.0/9.0)) as i32
}
