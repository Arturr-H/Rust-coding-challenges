/*- Global allowances -*/
#![allow(
    dead_code,
    unused_imports,
)]

/*- Imports -*/
use std::fmt::Debug;
use dotenv;

/*- Constants -*/
const BENCHMARK_ITERATIONS: usize = 1000;

/*- Imports -*/
#[path = "./hard/minimum_window_substring.rs"]
mod min_window_substring;

#[path = "./hard/median_of_two_sorted_arrays.rs"]
mod median_of_two_sorted_arrays;

#[path = "./hard/trapping_rain_water.rs"]
mod trapping_rain_water;

#[path = "./AOC-2022/input_getter.rs"]
mod input_getter;

/*- Challenges AOC-2022 -*/
#[path = "./AOC-2022/day1/challenge.rs"]
mod day1;

/*- Initialize -*/
fn main() -> () {
    dotenv::dotenv().unwrap();

    // input_getter::get_input(1);
    run(min_window_substring        ::run("pafbaidawajiddonaoie", "def"));
    run(median_of_two_sorted_arrays ::run(&[1, 2], &[3]));
    run(trapping_rain_water         ::run(&[4,2,0,3,2,5]));
    run(day1                        ::run());
    run(day1                        ::run_2());

    /*- Time function call -*/
    // let start = std::time::Instant::now();

    // /*- Move any function which we want to time here -*/
    // for _ in 0..BENCHMARK_ITERATIONS {
    //     trapping_rain_water::run(&[4,2,0,3,2,5]);
    // };

    // println!("Average time after {BENCHMARK_ITERATIONS} calls: {}", std::time::Instant::now().duration_since(start).as_secs_f32() / BENCHMARK_ITERATIONS as f32);
}

/*- Just debug output of function -*/
pub fn run<T: Debug>(t:T) -> () {
    println!("{t:?}");
}
pub fn test<T: Fn(P) -> A, P, A:Debug>(f: T, p: P) -> () {
    run(f(p))
}
