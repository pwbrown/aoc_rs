use utils::get_input_data;

#[tokio::main]
async fn main() {
    // get input data
    let data = get_input_data("2015", "9").await.trim().to_string();
}
