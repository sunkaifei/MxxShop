use std::collections::HashSet;

use base64::Engine;
use base64::engine::general_purpose;
use uuid::Uuid;

pub fn generate_random_code(length: usize) -> Option<String> {
    if length < 5 {
        return None;
    }

    let mut unique_codes = HashSet::new();

    loop {
        // Generate a random UUID and convert it to string format
        let uuid_str = Uuid::new_v4().to_string();

        // Convert UUID to base64 encoded string
        let code = general_purpose::URL_SAFE_NO_PAD.encode(uuid_str);
        // Extract substring of the desired length from the base64 encoded string
        let code = code.chars().take(length).collect::<String>();

        // Check if this code has already been generated
        if unique_codes.contains(&code) {
            continue;
        }

        // Add the new code to the set of unique codes
        unique_codes.insert(code.clone());

        // Return the new code
        return Some(code);
    }
}
