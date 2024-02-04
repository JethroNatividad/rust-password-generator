// Write a program that generates password with special characters, and numbers
// Inputs: length, has_lowercase, has_uppercase, has_special_characters, has_numbers
// Process: generate random password
// Output: random password

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let password = generate_password(12, true, true, true, true);

        // Check length
        assert_eq!(password.len(), 12);

        // Check for lowercase, uppercase, numbers, and special characters
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_numeric = password.chars().any(|c| c.is_numeric());
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()[]{};:,.<>?/\\|".contains(c));

        assert!(
            has_lowercase,
            "Password should contain at least one lowercase letter"
        );
        assert!(
            has_uppercase,
            "Password should contain at least one uppercase letter"
        );
        assert!(has_numeric, "Password should contain at least one number");
        assert!(
            has_special,
            "Password should contain at least one special character"
        );
    }
}
fn main() {
    println!("Hello, world!");
}
