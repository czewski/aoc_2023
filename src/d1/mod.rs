use std::fs;

pub fn test() {
    let contents =
        fs::read_to_string("src/d1/test.txt").expect("Should have been able to read the file");

    println!("Text:\n{contents}");

    // Extract first and last numbers
    let numbers = get_numbers(contents);
    println!("Numbers: {:?} ", numbers)

    // Get the first and last character of every line

    // Convert the characters to numbers (concatenating)

    // Sum each line
}

fn get_numbers(input: String) -> Vec<u32> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}
