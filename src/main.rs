use rand::Rng;

// Write a program that generates password with special characters, and numbers
// Inputs: length, has_lowercase, has_uppercase, has_special_characters, has_numbers
// Process: generate random password
// Output: random password

fn get_random_character(string: &str) -> char {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..string.chars().count());
    string.chars().nth(idx).unwrap()
}

fn generate_password(
    length: i32,
    has_lowercase: bool,
    has_uppercase: bool,
    has_numbers: bool,
    has_special_characters: bool,
) -> String {
    // if length < 5 error
    if length < 5 {
        return "".to_string();
    }

    let lowercase: &str = "abcdefghijklmnopqrstuvwxyz";
    let uppercase: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers: &str = "0123456789";
    let special_characters: &str = "!@#$%^&*()[]{};:,.<>?/\\|";

    // construct characters
    let mut characters: String = "".to_string();
    let mut password: String = "".to_string();

    let mut n_choices: i32 = 0;

    // add to characters based on condition
    // construct initial password, add atleast 1 based on condition,
    if has_lowercase {
        characters += lowercase;
        password.push(get_random_character(lowercase));
        n_choices += 1;
    }
    if has_uppercase {
        characters += uppercase;
        password.push(get_random_character(uppercase));
        n_choices += 1;
    }
    if has_numbers {
        characters += numbers;
        password.push(get_random_character(numbers));
        n_choices += 1;
    }
    if has_special_characters {
        characters += special_characters;
        password.push(get_random_character(special_characters));
        n_choices += 1;
    }

    if n_choices < 1 {
        return "".to_string();
    }

    // loop length - initial length
    for _ in 0..(length - n_choices) {
        // add to password
        password.push(get_random_character(characters.as_str()));
        // shuffle password once more
        // return password
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        for i in 5..20 {
            let password = generate_password(i, true, true, true, true);
            assert_eq!(password.len() as i32, i, "Password length should be {}", i);
        }
    }

    #[test]
    fn test_password_no_lowercase() {
        let password = generate_password(12, false, true, true, true);
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        assert!(
            !has_lowercase,
            "Password should not contain lowercase letters"
        );
    }

    #[test]
    fn test_password_no_uppercase() {
        let password = generate_password(12, true, false, true, true);
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        assert!(
            !has_uppercase,
            "Password should not contain uppercase letters"
        );
    }

    #[test]
    fn test_password_no_numbers() {
        let password = generate_password(12, true, true, false, true);
        let has_numeric = password.chars().any(|c| c.is_numeric());
        assert!(!has_numeric, "Password should not contain numbers");
    }

    #[test]
    fn test_password_no_special_characters() {
        let password = generate_password(12, true, true, true, false);
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()[]{};:,.<>?/\\|".contains(c));
        assert!(
            !has_special,
            "Password should not contain special characters"
        );
    }

    #[test]
    fn test_password_all_false() {
        let password = generate_password(12, false, false, false, false);
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_numeric = password.chars().any(|c| c.is_numeric());
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()[]{};:,.<>?/\\|".contains(c));

        assert!(
            !has_lowercase && !has_uppercase && !has_numeric && !has_special,
            "Password should not contain any characters"
        );
    }

    #[test]
    fn test_password_length_zero() {
        let password = generate_password(0, true, true, true, true);
        assert_eq!(password.len(), 0, "Password length should be 0");
    }

    #[test]
    fn test_password_length_negative() {
        let password = generate_password(-5, true, true, true, true);
        assert_eq!(password.len(), 0, "Password length should be 0");
    }
}
fn main() {
    println!("Hello, world!");
}
