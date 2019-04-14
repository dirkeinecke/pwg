//! # pwg
//!
//! With `pwg` you can easily create random passwords. You can specify how long the password should be and whether it should also contain capital letters, numbers and special characters.

extern crate rand;

use rand::prelude::*;

/// The ingredients from which the password is generated are provided here.
///
/// - lowercase letters
/// - upper-case letters
/// - numbers
/// - symbols (special characters)
struct Pool<'a> {
    lowercase_letters: [&'a str; 26],
    uppercase_letters: [&'a str; 26],
    numbers: [&'a str; 10],
    symbols: [&'a str; 8],
}

/// Easily create random passwords. You can specify how long the password should be and whether it should also contain capital letters, numbers and special characters.
///
/// # Examples
///
/// ```rust
/// /*
///   A password with 20 characters is generated.
///   It contains lowercase letters (default),
///   uppercase letters, numbers and symbols.
/// */
/// let password = pwg::new(20, &["uppercase", "numbers", "symbols"]);
/// println!("{}", password);
/// ```
///
/// ```rust
/// /* A password with 10 characters (lower case letters) is generated. */
/// let password = pwg::new(10, &[]);
/// println!("{}", password);
/// ```
///
/// ```rust
/// /*
///   A password with 20 characters is generated.
///   It contains lowercase letters (default) and uppercase letters.
/// */
/// let password = pwg::new(20, &["uppercase"]);
/// println!("{}", password);
/// ```
///
/// ```rust
/// /*
///   A password with 20 characters is generated.
///   It contains lowercase letters (default) and numbers.
/// */
/// let password = pwg::new(20, &["numbers"]);
/// println!("{}", password);
/// ```
///
/// ```rust
/// /*
///   A password with 20 characters is generated.
///   It contains lowercase letters (default) and symbols.
/// */
/// let password = pwg::new(20, &["symbols"]);
/// println!("{}", password);
/// ```
pub fn new(length: usize, ingredients: &[&str]) -> String {
    let pool = Pool {
        lowercase_letters: ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"],
        uppercase_letters: ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"],
        numbers: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        symbols: ["!", "§", "$", "%", "&", "?", "@", "#"],
    };

    let mut rng = thread_rng();

    let mut password = String::new();

    while password.len() < length {
        let mut random_strings = ["", "", "", ""];

        random_strings[0] = pool.lowercase_letters.choose(&mut rng).unwrap();

        if ingredients.contains(&"uppercase") {
            random_strings[1] = pool.uppercase_letters.choose(&mut rng).unwrap();
        }

        if ingredients.contains(&"numbers") {
            random_strings[2] = pool.numbers.choose(&mut rng).unwrap();
        }

        if ingredients.contains(&"symbols") {
            random_strings[3] = pool.symbols.choose(&mut rng).unwrap();
        }

        let random_character = random_strings.choose(&mut rng).unwrap();

        password.push_str(random_character);
    }

    password
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length() {
        let password = new(10, &[]);
        assert_eq!(10, password.len());
    }
}