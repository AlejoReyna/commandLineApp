use std::io; 

// Function that reads text from terminal
fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin");
    
    let trimmed = input_text.trim();
    println!("You entered: {}", trimmed); 
}
