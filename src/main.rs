use frunk::monoid::Monoid;

// Custom `combine_all` function
fn combine_all<T: Monoid>(items: Vec<T>) -> T {
    items.iter().fold(T::empty(), |acc, item| acc.combine(item))
}

fn main() {
    // Example with strings (String has a Monoid implementation)
    let strings = vec!["Hello, ".to_string(), "World".to_string(), "!".to_string()];
    let combined_strings = combine_all(strings);
    println!("Combined Strings: {}", combined_strings);

    // Example with integers (i32 has a Monoid implementation for summing)
    let numbers = vec![1, 2, 3, 4, 5];
    let combined_numbers = combine_all(numbers);
    println!("Sum of Numbers: {}", combined_numbers);
}
