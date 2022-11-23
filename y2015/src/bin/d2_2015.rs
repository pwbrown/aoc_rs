use utils::get_input_data;

#[tokio::main]
async fn main() {
    // Get input data
    let data = get_input_data("2015", "2").await;

    let mut total_paper: i32 = 0;
    let mut total_ribbon: i32 = 0;

    // Process each box
    for line in data.lines() {
        let lwh: Vec<i32> = line.split('x').map(|d| d.parse().unwrap()).collect();
        let areas = vec![lwh[0] * lwh[1], lwh[1] * lwh[2], lwh[2] * lwh[0]];
        let min_area = areas.iter().min().unwrap();
        let max_length = lwh.iter().max().unwrap();

        let surface_area = areas.iter().fold(0, |tot, area| tot + area * 2);

        let surface_length: i32 = lwh.iter().fold(0, |tot, len| tot + len * 2);

        let volume: i32 = lwh.iter().product();

        total_paper += surface_area + min_area;
        total_ribbon += surface_length - max_length * 2 + volume;
    }

    println!("Part 1 Answer: {} square feet of paper", total_paper);
    println!("Part 2 Answer: {} feet of ribbon", total_ribbon);
}
