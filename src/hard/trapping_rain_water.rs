/// Given n non-negative integers representing an elevation map where the
/// width of each bar is 1, compute how much water it can trap after raining.
pub fn run<'a>(height:&'a [u32]) -> () {
    let max_bar = *height.iter().max().unwrap();
    let heightmap_len = height.len();
    let mut total:u32 = 0;

    /*- Iterate -*/
    for y in 0..max_bar {
        // println!("{y}");
        for x in 0..heightmap_len {
            /*- Cant't be trapped if near boundaries -*/
            if x == 0 || x == heightmap_len { continue; };

            /*- If is inside bar -*/
            if height[x] >= y { continue; };

            /*- Check <-x-> for barriers -*/
            match check(height, x, y as usize, max_bar) {
                true => total += 1,
                false => ()
            };
        }
    }

    println!("{total}");
}
fn check(height: &[u32], x:usize, y:usize, max:u32) -> bool {
    /*- Check <- -*/
    let mut _x = x.clone();
    loop {
        if _x == 0 { return false };
        _x -= 1;

        /*- Collide -*/
        if height[_x] >= max-y as u32 { break };
    };

    /*- Check -> -*/
    let mut _x = x.clone();
    loop {
        if _x == height.len() - 1 { return false };
        _x += 1;

        /*- Collide -*/
        if height[_x] >= max-y as u32 { break };
    };

    true
}