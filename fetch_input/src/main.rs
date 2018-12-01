extern crate reqwest;

use std::error::Error;
use std::fs;
use std::io;

fn load_session_cookie() -> Result<String, io::Error> {
    fs::read_to_string(".session_cookie")
        .map(|s| s.trim().to_string())
}

fn input_file_name(_year: u32, day: u32) -> String {
    format!("../{:02}/input", day)
}

fn fetch_input(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::Client::new();
    let session_cookie = load_session_cookie()?;
    let text = client.get(&url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send()
        .expect("request failed")
        .error_for_status()?
        .text();
    // text.map_err(Box::new)
    match text {
        Ok(t) => Ok(t),
        Err(e) => Err(Box::new(e)),
    }
}

fn main() {
    let year = 2018;
    for day in 1..26 {
        let input_file_name = input_file_name(year, day);
        if fs::metadata(&input_file_name).is_err() {
            println!("Input for {} day {} does not exist, fetching...",
                year, day);
            let contents = match fetch_input(year, day) {
                Ok(contents) => contents,
                Err(err) => {
                    println!("Could not fetch input for {} day {}: {}",
                        year, day, err);
                    break;
                }
            };
            fs::write(&input_file_name, &contents)
                .expect(&format!("could not write file {}", &input_file_name));
            println!("Wrote {}", &input_file_name);
        }
    }
}
