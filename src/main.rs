// Write a program that generates password with special characters, and numbers
// Inputs: length, has_lowercase, has_uppercase, has_special_characters, has_numbers
// Process: generate random password
// Output: random password

fn generate_password(
    length: i64,
    has_lowercase: bool,
    has_uppercase: bool,
    has_special_characters: bool,
    has_numbers: bool,
) -> String {
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        for i in 1..20 {
            let password = generate_password(i, true, true, true, true);
            assert_eq!(password.len(), i, "Password length should be {}", i);
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
