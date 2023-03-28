pub fn caesar_translate(input_text: String, mode: String, key: isize) -> String {
    // Purpose:    Does the Caesar cipher logic.
    // Parameters: input_text to translate, mode (encrypt or decrypt, and key (in size).
    // User Input: None.
    // Prints:     None.
    // Returns:    Translated text as a std::String.
    // Modifies:   Nothing.
    // Calls:      Pure rust, no imports. Hint: rem_euclid
    // Tests:      ./unit_tests/*
    // Status:     Do this one!
    // asserteq!(caesar_translate("abc".to_string, "encrypt".to_string(), 1), "bcd".to_string())
    // asserteq!(caesar_translate("bcd".to_string, "decrypt".to_string(), 1), "abc".to_string())

    // Every possible symbol that can be encrypted:
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";
    let symbol_vec: Vec<char> = symbols.chars().collect();
    let mut translated = String::new();
    let text = input_text.chars();
    let encrypt = "encrypt";
    let decrypt = "decrypt";

    for c in text {
        if symbols.contains(c) {
            let index = symbol_vec.iter().position(|&x| x == c).unwrap();
            let index = index as isize;
            if mode == encrypt {
                let new_index = (((index + key) % 66) + 66) % 66;
                let new_index = new_index as usize;
                translated.push_str(&symbol_vec[new_index].to_string());
            } else if mode == decrypt {
                let new_index = (((index - key) % 66) + 66) % 66;
                let new_index = new_index as usize;
                translated.push_str(&symbol_vec[new_index].to_string());
            }
        } else {
            translated.push_str(&c.to_string());
        }
    }
    let mut text = translated.to_string();
    text.trim();
    text.to_string()
}
