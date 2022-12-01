use utils::get_input_data;

#[tokio::main]
async fn main() {
    let data = get_input_data("2022", "1").await.trim().to_string();

    let mut elves_cals: Vec<i32> = vec![];

    let mut current = 0;

    for line in data.lines() {
        if line.is_empty() {
            elves_cals.push(current);
            current = 0;
        } else {
            let cal: i32 = line.parse().unwrap();
            current += cal;
        }
    }

    elves_cals.sort();
    println!("Part 1 Answer: {} calories", elves_cals.last().unwrap());
    println!("Part 2 Answer: {} calories", elves_cals.iter().rev().take(3).sum::<i32>());
}