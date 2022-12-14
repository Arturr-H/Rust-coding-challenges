/*- Global allowances -*/
#![allow(
    dead_code,
    unused_imports,
)]

/*- Imports -*/
use std::fmt::Debug;
use dotenv;

/*- Constants -*/
const BENCHMARK_ITERATIONS: usize = 1000000;

/*- Imports -*/
#[path = "./hard/minimum_window_substring.rs"]
mod min_window_substring;

#[path = "./hard/median_of_two_sorted_arrays.rs"]
mod median_of_two_sorted_arrays;

#[path = "./hard/trapping_rain_water.rs"]
mod trapping_rain_water;

#[path = "./medium/number_of_islands.rs"]
mod number_of_islands;

#[path = "./hard/integer_to_english_words.rs"]
mod integer_to_english_words;

#[path = "./hard/self_crossing.rs"]
mod self_crossing;

#[path = "./arrays/get_sum.rs"]
mod get_sum;

/*- Initialize -*/
fn main() -> () {
    dotenv::dotenv().unwrap();
    // input_getter::get_input(1);

    run(min_window_substring        ::run("pafbaidawajiddonaoie", "def"));
    run(median_of_two_sorted_arrays ::run(&[1, 2], &[3]));
    run(trapping_rain_water         ::run(&[4,2,0,3,2,5]));
    run(number_of_islands           ::run(vec![vec![1, 1, 0, 0, 0], vec![1, 1, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 1, 1]]));
    run(integer_to_english_words    ::run(2935));
    run(self_crossing               ::run(&[2,1,1,2]));
    run(get_sum                     ::run::nested(&[2,2,1,1], 4));
    run(get_sum                     ::run::backchecked(&[2,2,1,1], 4));

    /*- Time function call -*/
    let start = std::time::Instant::now();

    /*- Move any function which we want to time here -*/
    for _ in 0..BENCHMARK_ITERATIONS {
        get_sum                     ::run::backchecked(&[2,2,1,1,5,8,3,1,4,7,434,1232,4,42,134,56,75,453423,23243,12], 490);
    };
    // nested: 0.000004373027s,


    println!("Average time after {BENCHMARK_ITERATIONS} calls: {}s", std::time::Instant::now().duration_since(start).as_secs_f32() / BENCHMARK_ITERATIONS as f32);
}

/*- Just debug output of function -*/
pub fn run<T: Debug>(t:T) -> () {
    println!("{t:?}");
}
pub fn test<T: Fn(P) -> A, P, A:Debug>(f: T, p: P) -> () {
    run(f(p))
}
