const ENG_WORDS:&[&[&str]] = &[
    &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"],
    &["ten", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "ninety"],
    &["hundred", "thousand", "million"],
];

/// Convert a non-negative integer num to its English words representation.
pub fn run(number:usize) -> String {
    if number == 0 { return String::from("zero") };

    match number.to_string().len() {
        1 => ENG_WORDS[0][number-1].to_string(),
        2 => _0_99_(number),
        3 => _100_999_(number),
        4 => _1000_9999_(number),
        _ => unimplemented!()
    }
}
fn _0_99_(num:usize) -> String {
    match num {
        10..=19 => ENG_WORDS[0][num-1].to_string(),
        _ => {
            let mut fin = String::new();
            fin.push_str(ENG_WORDS[1][num/10 - 1]);
            fin.push_str(if num%10 == 0 { "" } else { ENG_WORDS[0][num%10 - 1] });
            fin
        }
    }
}
fn _100_999_(num:usize) -> String { format!("{} hundred and {}", ENG_WORDS[0][num/100-1].to_string(), _0_99_(num % 100)) }
fn _1000_9999_(num:usize) -> String { format!("{} thousand {}", ENG_WORDS[0][num/1000-1].to_string(), _100_999_(num % 1000)) }