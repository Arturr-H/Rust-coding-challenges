
/// Given two strings s and t of lengths m and n respectively, return the minimum
/// window substring of s such that every character in t (including duplicates)
/// is included in the window. If there is no such substring, return the empty string "".
pub fn run(s: &str, t:&str) -> () {
    let mut end = Vec::new();
    for (index, _char) in s.chars().enumerate() {
        if t.contains(_char) {
            match run_inner(s, t.len(), t, vec![(_char, index)]) {
                Some(e) => end = vec![end, e].concat(),
                None => ()
            }
        };
    };

    /*- Find min window -*/
    let mut final_:(String, usize) = (String::new(), 0);
    for (index, tuples) in end.iter().enumerate() {
        let indexes = tuples.iter().map(|e| e.1).collect::<Vec<usize>>();
        let chars = tuples.iter().map(|e| e.0).collect::<String>();
        let max = indexes.iter().max().unwrap();
        let min = indexes.iter().min().unwrap();

        if index == 0 {
            final_ = (chars, max - min)
        }else if max - min < final_.1 {
            final_ = (chars, max - min)
        }
    }
    println!("{final_:?}");
}
fn run_inner<'a>(s:&'a str, depth:usize, t:&str, curr:Vec<(char, usize)>) -> Option<Vec<Vec<(char, usize)>>> {
    if depth <= 0 { return None };
    if depth == 1 {
        for window_char in t.chars() {
            if !curr.iter().map(|e| e.0).collect::<Vec<char>>().contains(&&window_char) {
                return None;
            };
        };
        Some(vec![curr])
    }else {
        let mut end = Vec::new();
        for (index, _char) in s.chars().enumerate() {
            if t.contains(_char) {
                match run_inner(
                    s,
                    depth - 1,
                    t, 
                    vec![curr.clone(), vec![(_char.clone(), index)]].concat(),
                ) {
                    Some(e) => end = vec![end, e].concat(),
                    None => ()
                };
            };
        };

        if end.is_empty() { None }
        else { Some(end) }
    }

}
