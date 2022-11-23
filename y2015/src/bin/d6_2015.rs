use utils::get_input_data;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

type Point = (i32, i32);

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2015", "6").await.trim().to_string();

    // Holds lighted status and brightness level for all pixels
    let mut lights = HashMap::<String, (bool, i32)>::new();

    for line in data.lines() {
        // Determine the instruction and grid points (corners)
        let (inst, point1, point2) = parse_line(line);
        // Iterate through each point in the grid
        for x in point1.0..=point2.0 {
            for y in point1.1..=point2.1 {
                // Generate a lights pixel string (key)
                let pixel = format!("{},{}", x, y);
                // Set default status values
                let mut is_lit = false;
                let mut brightness = 0;
                // Derive status values from an existing pixel if possible
                if lights.contains_key(&pixel) {
                    (is_lit, brightness) = lights.remove(&pixel).unwrap();
                }
                // Handle each instruction's actions
                match inst {
                    Instruction::TurnOn => {
                        is_lit = true;
                        brightness += 1;
                    }
                    Instruction::TurnOff => {
                        is_lit = false;
                        if brightness > 0 {
                            brightness -= 1;
                        }
                    }
                    Instruction::Toggle => {
                        is_lit = !is_lit;
                        brightness += 2;
                    }
                }
                // (Re)insert light into the overall grid
                if is_lit || brightness > 0 {
                    lights.insert(pixel, (is_lit, brightness));
                }
            }
        }
    }

    // Determine the total number of lighted pixels, and the total brightness
    let results = lights
        .values()
        .fold((0, 0), |(mut tot_lit, tot_brt), (is_lit, brightness)| {
            if *is_lit {
                tot_lit += 1;
            }
            (tot_lit, tot_brt + *brightness)
        });

    println!("Part 1 Answer: {} lights are on", results.0);
    println!("Part 2 Answer: {} total brightness", results.1);
}

/// Parse line into an instruction and coordinates
fn parse_line(line: &str) -> (Instruction, Point, Point) {
    let mut words = line.split_ascii_whitespace();
    let mut inst = Instruction::Toggle;
    if let Some(first) = words.next() {
        if first == "toggle" {
            inst = Instruction::Toggle;
        } else if let Some(second) = words.next() {
            inst = match second {
                "on" => Instruction::TurnOn,
                _ => Instruction::TurnOff,
            }
        }
    }
    let coords1: Vec<&str> = words.next().unwrap().split(',').collect();
    let point1: (i32, i32) = (coords1[0].parse().unwrap(), coords1[1].parse().unwrap());
    // Skip the "through" word
    words.next();
    let coords2: Vec<&str> = words.next().unwrap().split(',').collect();
    let point2: (i32, i32) = (coords2[0].parse().unwrap(), coords2[1].parse().unwrap());
    // Return line results
    (inst, point1, point2)
}
