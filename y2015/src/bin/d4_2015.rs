use utils::get_input_data;

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2015", "4").await.trim().to_string();

    let mut n = 0;

    let mut five = -1;
    let mut six = -1;

    loop {
        n += 1;
        let hashed = md5::compute(format!("{}{}", data, n));
        let hash_str = format!("{:x}", hashed);
        if hash_str.starts_with("00000") && five == -1 {
            five = n;
        }
        if hash_str.starts_with("000000") && six == -1 {
            six = n;
        }
        if five != -1 && six != -1 {
            break;
        }
    }

    println!("Part 1 Answer: {} iterations", five);
    println!("Part 2 Answer: {} iterations", six);
}
