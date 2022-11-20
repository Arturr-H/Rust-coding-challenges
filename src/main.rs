/*- Global allowances -*/
#![allow(
    dead_code,
    unused_imports,
)]

/*- Imports -*/
#[path = "./hard/minimum_window_substring.rs"]
mod min_window_substring;

#[path = "./hard/median_of_two_sorted_arrays.rs"]
mod median_of_two_sorted_arrays;

/*- Initialize -*/
fn main() -> () {
    min_window_substring::run("pafbaidawajiddonaoie", "def");
    median_of_two_sorted_arrays::run(&[1, 2], &[3]);
}

