mod crabby_caesar;

fn main() {
    // Purpose:    IO, and calls your functions.
    // Parameters: None
    // User Input: Input text to translate
    // Prints:     Print result
    // Returns:    Nothing
    // Modifies:   Nothing outside its scope
    // Calls:      std::stuff...
    // Tests:      stdio_tests/
    // Status:     Do this one.

    let mut text = String::new();
    println!("Enter a string to translate:");
    std::io::stdin().read_line(&mut text);
    let mut method = String::new();
    println!("encrypt or decrypt?");
    std::io::stdin().read_line(&mut method);
    let mut key = String::new();
    println!("What is your key?");
    std::io::stdin().read_line(&mut key);
    let key: isize = key.trim().parse().expect("");
    let text: String = text.to_string();
    let method: String = method.trim().to_string();
    println!(
        "{}",
        crate::crabby_caesar::caesar_translate(text, method, key)
    );
}
