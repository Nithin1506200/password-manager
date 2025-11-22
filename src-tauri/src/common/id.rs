use macro_utils::gen_id;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ID {
    Profile,
    Passwords,
}

#[gen_id(length = 4, prefix = "AxC")]
pub struct ProfileId;

// Add a test to verify the macro works correctly
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_id_generation() {
        let id = ProfileId::new();
        let id_str = id.to_string();

        println!("Generated ID: {}", id_str);

        // Verify the prefix
        assert!(id_str.starts_with("ABC"));

        // Verify the length (prefix + random chars)
        assert_eq!(id_str.len(), 11); // 3 chars for "ABC" + 8 random chars
    }
}
