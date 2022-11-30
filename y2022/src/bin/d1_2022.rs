use utils::get_input_data;

#[tokio::main]
async fn main() {
    let data = get_input_data("2022", "1").await.trim().to_string();
}