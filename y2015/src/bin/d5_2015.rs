use std::collections::HashMap;

use utils::get_input_data;

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2015", "5").await.trim().to_string();

    let invalid_pairs = vec![
        String::from("ab"),
        String::from("cd"),
        String::from("pq"),
        String::from("xy"),
    ];
    let vowel_chars = vec!['a', 'e', 'i', 'o', 'u'];
    let mut p1_nice_strings = 0;
    let mut p2_nice_strings = 0;

    for line in data.lines() {
        let mut vowels = Vec::new();
        let mut pairs = HashMap::new();
        let mut has_double = false;
        let mut has_invalid_pair = false;
        let mut has_sandwich = false;

        for (i, char) in line.chars().enumerate() {
            /* Check for vowel */
            if vowel_chars.contains(&char) {
                vowels.push(char);
            }

            // Handle character pair
            if i > 0 {
                let prev1 = line.chars().nth(i - 1).unwrap();
                // Check for matching characters
                if prev1 == char {
                    has_double = true;
                }
                let pair = format!("{}{}", prev1, char);
                // Check against invalid pairs
                if invalid_pairs.contains(&pair) {
                    has_invalid_pair = true;
                }
                let mut start: i32 = -2;
                let mut count: i32 = 0;
                // Check for existing matching pair
                if pairs.contains_key(&pair) {
                    (start, count) = pairs.remove(&pair).unwrap();
                }
                // Insert the pair if it does not overlap with an existing pair
                if i as i32 > start + 1 {
                    count += 1;
                    pairs.insert(pair, (i as i32, count));
                } else {
                    // Reinsert removed pair
                    pairs.insert(pair, (start, count));
                }
                // Check for sandwich pattern
                if i > 1 {
                    let prev2 = line.chars().nth(i - 2).unwrap();
                    if prev2 == char {
                        has_sandwich = true;
                    }
                }
            }
        }

        let valid_pairs = pairs
            .iter()
            .filter(|(_k, (_s, count))| *count >= 2)
            .map(|(k, _v)| String::from(k));

        // Part 1 nice string qualifications
        if vowels.len() >= 3 && has_double && !has_invalid_pair {
            p1_nice_strings += 1;
        }

        // Part 2 nice string qualifications
        if has_sandwich && valid_pairs.count() > 0 {
            p2_nice_strings += 1;
        }
    }

    println!("Part 1 Answer: {} nice strings", p1_nice_strings);
    println!("Part 2 Answer: {} nice strings", p2_nice_strings);
}
