/*- Imports -*/
use reqwest;
use std::{fs, io::Write};

/*- Initialize -*/
pub fn get_input(day:u8) -> () {
    let session = std::env::var("SESSION").unwrap();
    let text = reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/2021/day/{day}/input"))
        .header("Cookie", format!("session={session}"))
        .send().unwrap().text().unwrap();
    
    let mut file = fs::File::create(format!("day{day}/input.txt")).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}
pub fn get_text(day:u8) -> String {
    fs::read_to_string(format!("day{day}/input.txt")).unwrap()
}