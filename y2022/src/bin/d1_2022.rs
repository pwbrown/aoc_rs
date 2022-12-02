use utils::get_input_data;

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2022", "1").await.trim().to_string();

    // Create a list to hold the total calories for each elf
    let mut elves_cals: Vec<i32> = vec![];

    // Holds the total calories for the currently selected elf
    let mut current = 0;

    for line in data.lines() {
        // An empty line is a breakpoint for a new elf (add the current, and then reset)
        if line.is_empty() {
            elves_cals.push(current);
            current = 0;
        } else {
            // Just push the calories into the current elf's total
            let cal: i32 = line.parse().unwrap();
            current += cal;
        }
    }
    // Pushes the final elf's calories
    elves_cals.push(current);

    // Sort so that the highest calories appear at the end of the vector
    elves_cals.sort();

    // Part 1 is looking for the highest colories value (the last element)
    println!("Part 1 Answer: {} calories", elves_cals.last().unwrap());

    // Part 2 is looking for the total of the 3 highest calories (sum of the last three values)
    println!(
        "Part 2 Answer: {} calories",
        elves_cals.iter().rev().take(3).sum::<i32>()
    );
}
