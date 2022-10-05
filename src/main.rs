/*- Global allowings -*/
#![allow(
    dead_code
)]

/*- Imports -*/
mod challenges;

/*- Initialize -*/
fn main() -> () {
    println!("{:?}", challenges::how_much_is_true(vec![false, true, true, false, true]));
    println!("{:?}", challenges::find_seven(vec![1, 125, 123, 654, 1471]));
    println!("{:?}", challenges::find_boomerangs(vec![1, 125, 1, 125, 1471]));
    println!("{:?}", challenges::track_robot(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    let zeroes = &mut vec![1, 6, 2, 0, 2, 0, 6, 8, 1, 0];
    challenges::move_zeroes(zeroes);
    println!("{:?}", zeroes);
    println!("{:?}", challenges::two_powers_of_two(2048));
    println!("{:?}", challenges::three_sum(&vec![1, 2, 34, 5, 7, 8, 1, -1, 0]));
}