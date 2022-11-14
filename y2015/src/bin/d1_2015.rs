use utils::get_input_data;

#[tokio::main]
async fn main() {
  // Get input data
  let data = get_input_data("2015", "1").await;
  
  // Set starting floor
  let mut floor: i32 = 0;
  let mut pos: i32 = -1;

  for (i, c) in data.chars().enumerate() {
    if c == '(' {
      floor += 1
    } else if c == ')' {
      floor -= 1
    }

    if floor < 0 && pos < 0 {
      pos = i as i32 + 1;
    }
  }

  println!("Part 1 Answer: {}", floor);
  println!("Part 2 Answer: {}", pos);
}