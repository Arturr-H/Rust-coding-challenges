use crate::input_getter;

/// To do this, count the number of times a depth
/// measurement increases from the previous measurement.
/// (There is no measurement before the first measurement.) 
pub fn run() -> u32 {
    let mut times_increase:u32 = 1;
    let lines = input_getter::get_text(1);
    let l = lines.split("\n").collect::<Vec<&str>>();
    for idx in 1..lines.split("\n").count() {
        let prev = l[idx-1];
        let curr = l[idx];

        if curr > prev { times_increase+=1 }
    };

    times_increase
}
pub fn run_2() -> u32 {
    let mut times_increase:u32 = 1;
    let lines = input_getter::get_text(1);
    let l = lines.split("\n").collect::<Vec<&str>>();
    for idx in 2..lines.split("\n").count()-2 {
        let prev_2 = l[idx-2].parse::<u32>().unwrap();
        let prev = l[idx-1].parse::<u32>().unwrap();
        let curr = l[idx].parse::<u32>().unwrap();
        let next = l[idx + 1].parse::<u32>().unwrap();

        if (prev + curr + next) < (prev_2 + prev + curr) { times_increase+=1 }
    };

    times_increase
}