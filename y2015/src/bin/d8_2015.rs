use utils::get_input_data;

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2015", "8").await;

    let mut code_chars = 0;
    let mut mem_chars = 0;
    let mut enc_chars = 0;

    for line in data.lines() {
        code_chars += line.len();
        let mut chars = line.chars().peekable();
        while chars.peek().is_some() {
            let curr = chars.next().unwrap();
            match curr {
                // Double quote chars at beginning and end of string
                '"' => {
                    enc_chars += 3; // '"/"' or '\""' encoded
                }
                // Escape character '\'
                '\\' => {
                    enc_chars += 2; // '\\' encoded
                    match chars.next().unwrap() {
                        // Backslash
                        '\\' => {
                            mem_chars += 1;
                            enc_chars += 2; // '\\' encoded
                        }
                        // Double quote
                        '"' => {
                            mem_chars += 1;
                            enc_chars += 2; // '\"' encoded
                        }
                        // Hexidecimal
                        'x' => {
                            chars.next();
                            chars.next();
                            mem_chars += 1;
                            enc_chars += 3;
                        }
                        _ => panic!("Unknown escape character"),
                    }
                }
                _ => {
                    mem_chars += 1;
                    enc_chars += 1;
                }
            }
        }
    }

    println!(
        "Part 1 Answer: {} character difference",
        code_chars - mem_chars
    );

    println!(
        "Part 2 Answer: {} character difference",
        enc_chars - code_chars
    )
}
