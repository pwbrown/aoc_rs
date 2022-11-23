use std::collections::HashMap;
use utils::get_input_data;

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2015", "3").await;

    // Year 1 stats (only 1 Santa)
    let mut y1_delivered = HashMap::<String, i32>::new();
    y1_delivered.insert(String::from("0,0"), 1);
    let mut y1_pos = (0, 0);

    // Year 2 stats (Santa + Robo Stanta)
    let mut y2_delivered = HashMap::<String, i32>::new();
    y2_delivered.insert(String::from("0,0"), 2);
    let mut y2_pos = vec![
        (0, 0), // Santa's position
        (0, 0), // Robo Santa's position
    ];
    let mut y2_turn = 0;

    for direction in data.chars() {
        match direction {
            '^' => {
                y1_pos.1 += 1;
                y2_pos[y2_turn].1 += 1;
            }
            '>' => {
                y1_pos.0 += 1;
                y2_pos[y2_turn].0 += 1;
            }
            'v' => {
                y1_pos.1 -= 1;
                y2_pos[y2_turn].1 -= 1;
            }
            '<' => {
                y1_pos.0 -= 1;
                y2_pos[y2_turn].0 -= 1;
            }
            _ => {}
        };

        // Handle year 1
        let y1_coord = format!("{},{}", y1_pos.0, y1_pos.1);
        let y1_presents = if y1_delivered.contains_key(&y1_coord) {
            y1_delivered.remove(&y1_coord).unwrap()
        } else {
            0
        };
        y1_delivered.insert(y1_coord, y1_presents + 1);

        // Handle year 2
        let y2_coord = format!("{},{}", y2_pos[y2_turn].0, y2_pos[y2_turn].1);
        let y2_presents = if y2_delivered.contains_key(&y2_coord) {
            y2_delivered.remove(&y2_coord).unwrap()
        } else {
            0
        };
        y2_delivered.insert(y2_coord, y2_presents + 1);
        y2_turn = (y2_turn + 1) % 2; // Increment turn
    }

    println!(
        "Part 1 Answer: {} houses delivered to",
        y1_delivered.keys().len()
    );
    println!(
        "Part 2 Answer: {} houses delivered to",
        y2_delivered.keys().len()
    );
}
