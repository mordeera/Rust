fn main() {
    // Manually Find largest element of list

   let number_list = vec![34, 50 , 22, 84, 100]; 
   let result = largest(&number_list);
   println!("The largest number is {}", result);

   let char_list = vec!['y', 'm', 'a', 'q'];
   let result = largest_copy(&char_list);
   println!("The largest char is {}", result);

   let string_list = vec!["A", "C", "B"]; 
   let result = largest_clone(&string_list); 
   println!("The largest string is {}", result); 

}

// Find largest element by reference 
// Compares references with each other and creates stack allocations via shallow copies
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_val = &list[0]; 
    for entry in list {
        if entry > largest_val {
            largest_val = entry; // Creates a shallow copy of the reference on the stack
        }
    }

    largest_val
}

// Find largest element by Copying 
// Creates just stack allocations via shallow copies 
fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_val = list[0];

    for &entry in list {
        if entry > largest_val {
            largest_val = entry; // Creates a shallow copy of the value on the stack
        }
    }

    largest_val
}

// Find largest element by Cloning 
// Clones every element of the list 
// Creates heap allocations when using heap types like string
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest_val = list[0].clone();

    for entry in list {
        let value = entry.clone(); 
        if value > largest_val {
            largest_val = value; // Move "value" int largest_val
        }
    }

    largest_val
}
