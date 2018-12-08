use std::error::Error;
use std::fmt::Display;
use std::env;
use std::fs;
use std::io;

pub fn main<P1, P2, R1, R2>(part1: P1, part2: P2)
    where P1: Fn(&str) -> R1, P2: Fn(&str) -> R2, R1: Display, R2: Display
{
    let day = env::current_exe().unwrap().file_stem().unwrap().to_str().unwrap().parse::<u32>().unwrap();
    let input = get_input(2018, day);
    println!("Answer to day {} part 1: {}", day, part1(&input));
    println!("Answer to day {} part 2: {}", day, part2(&input));
}

fn get_input(year: u32, day: u32) -> String {
    let input_file_name = input_file_name(year, day);
    fs::read_to_string(&input_file_name)
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("Input file {} could not be read, fetching...", input_file_name);
            let contents = fetch_input(year, day)?;
            fs::write(&input_file_name, &contents)?;
            Ok(contents)
        })
        .unwrap()
}

fn input_file_name(_year: u32, day: u32) -> String {
    format!("input/{:02}.in", day)
}

fn load_session_cookie() -> Result<String, io::Error> {
    fs::read_to_string(".session_cookie")
        .map(|s| s.trim().to_string())
}

fn fetch_input(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::Client::new();
    let session_cookie = load_session_cookie()?;
    client.get(&url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send()
        .expect("request failed")
        .error_for_status()?
        .text()
        .map_err(From::from)
}
