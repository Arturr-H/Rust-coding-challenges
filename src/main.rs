/*- Global allowances -*/
#![allow(
    dead_code,
    unused_imports,
)]

/*- Imports -*/
#[path = "./hard/minimum_window_substring.rs"]
mod min_window_substring;

/*- Initialize -*/
fn main() -> () {
    min_window_substring::run("pafbaidawajiddonaoie", "def");
}

