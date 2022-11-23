use std::fs;
use std::path::Path;

use clap::Parser;
use reqwest::header::COOKIE;

#[derive(Parser, Debug)]
pub struct InputArgs {
    /// Advent Of Code Session Token
    #[arg(short, long, value_name = "SESSION_TOKEN")]
    session: Option<String>,
}

/// Get the input data for the problem
pub async fn get_input_data(year: &'static str, day: &'static str) -> String {
    // Check the cache first
    if let Some(cache_content) = get_data_from_cache(year, day) {
        return cache_content;
    }

    // Get the session token
    let args = InputArgs::parse();
    let session = match args.session.as_deref() {
        Some(sess) => {
            // cache session string
            cache_session_token(sess);
            sess.to_string()
        }
        None => match get_session_from_cache() {
            Some(sess) => sess,
            None => panic!(
                "Session token was not found. Use the --session argument to initiate the cache."
            ),
        },
    };

    let data = match get_input_from_api(year, day, session).await {
        Ok(data) => data,
        _ => panic!("Unable to retrieve data"),
    };

    // cache data
    cache_data(year, day, &data);
    // return data
    data
}

/// Attempt to get session token from cache
fn get_session_from_cache() -> Option<String> {
    fs::read_to_string(".cache/session").ok()
}

/// Attempt to get the data from a cached file
fn get_data_from_cache(year: &'static str, day: &'static str) -> Option<String> {
    fs::read_to_string(format!(".cache/{}/{}.in", year, day)).ok()
}

/// Attempts to read the input data from the AOC input url directly
async fn get_input_from_api(
    year: &'static str,
    day: &'static str,
    session: String,
) -> reqwest::Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header(COOKIE, format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

/// Caches the session token
fn cache_session_token(session: &str) {
    if !Path::new(".cache").try_exists().unwrap() {
        fs::create_dir(".cache").unwrap();
    }
    fs::write(".cache/session", session).unwrap()
}

/// Caches the input data
fn cache_data(year: &'static str, day: &'static str, data: &str) {
    let dir = format!(".cache/{}", year);
    if !Path::new(&dir).try_exists().unwrap() {
        fs::create_dir_all(&dir).unwrap();
    }
    fs::write(format!("{}/{}.in", dir, day), data).unwrap()
}
