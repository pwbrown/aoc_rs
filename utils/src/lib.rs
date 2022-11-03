// use std::path::{Path, PathBuf};
use std::fs;
// use std::str::FromStr;

use clap::Parser;


#[derive(Parser, Debug)]
pub struct InputArgs {
    /// Advent Of Code Session Token
    #[arg(short, long, value_name = "SESSION_TOKEN")]
    session: Option<String>,
}

/// Get the input data for the problem
pub fn get_input_data(year: &'static str, day: &'static str) -> String {
    // Check the cache first
    if let Some(cache_content) = get_data_from_cache(year, day) {
        return cache_content;
    }

    panic!("Unable to get input data");
    
    // let args = InputArgs::parse();

    // if let Some(input) = args.input.as_deref() {
    //     return get_input_from_file(input)
    // }

    // get_input_from_api(args.session.as_deref())
}

/// Attempt to get the data from a cached file
fn get_data_from_cache(year: &'static str, day: &'static str) -> Option<String> {
    fs::read_to_string(format!(".cache/{}/{}.in", year, day)).ok()
}

/// Attempts to read the input data from the AOC input url directly
// fn get_input_from_api(session: Option<&str>) -> String {

// }












pub fn say_hello(name: &'static str) {
    println!("Hello {}", name)
}
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
