use utils::get_input_data;

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[tokio::main]
async fn main() {
    // Get the input data
    let data = get_input_data("2022", "2").await.trim().to_string();

    // Parse the data into a list of rounds
    let rounds = parse_rounds(&data);

    // Dictates the game's order of precedence
    let order = vec![Choice::Rock, Choice::Paper, Choice::Scissors];

    // Keep track of each part's total score
    let mut p1_score = 0;
    let mut p2_score = 0;

    for (opponent, player, outcome) in rounds {
        // Get the index of the choices within the order of precedence
        let opponent_order = order.iter().position(|&c| c == opponent).unwrap() as i32;
        let player_order = order.iter().position(|&c| c == player).unwrap() as i32;
        // Determine the order index of the win and lose outcomes based on the opponent's choice
        let win_order = (opponent_order + 1) % order.len() as i32; // Win is the next index (cycles to the beginning if needed)
        let lose_order = if opponent_order - 1 < 0 {
            // Lose is the previous index (cycles to the end if needed)
            order.len() as i32 - 1
        } else {
            opponent_order - 1
        };
        // Add the player's decision's point value (order of precendence in 1-indexed format)
        p1_score += player_order + 1;
        // Determine outcome score bonus based on player's choice in part 1 (Loss gets no points)
        if player_order == opponent_order {
            // Draw
            p1_score += 3;
        } else if player_order == win_order {
            // Win
            p1_score += 6;
        }
        // Tally outcome score for part 2, and determine choice score to produce the expected outcome
        match outcome {
            Outcome::Lose => {
                // Add lose choice value
                p2_score += lose_order + 1;
            }
            Outcome::Draw => {
                // Add draw bonus
                p2_score += 3;
                // Add the opponent's choice value
                p2_score += opponent_order + 1;
            }
            Outcome::Win => {
                // Add win bonus
                p2_score += 6;
                // Add win choice value
                p2_score += win_order + 1;
            }
        };
    }

    println!("Part 1 Answer: {} points", p1_score);
    println!("Part 2 Answer: {} points", p2_score);
}

fn parse_rounds(input: &str) -> Vec<(Choice, Choice, Outcome)> {
    let mut rounds = vec![];

    for line in input.lines() {
        let choices = line.split(' ').collect::<Vec<&str>>();
        rounds.push((
            str_to_choice(choices[0]),
            str_to_choice(choices[1]),
            str_to_outcome(choices[1]),
        ));
    }

    rounds
}

fn str_to_choice(choice_str: &str) -> Choice {
    match choice_str {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Unknown choice"),
    }
}

fn str_to_outcome(outcome_str: &str) -> Outcome {
    match outcome_str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unknown outcome"),
    }
}
