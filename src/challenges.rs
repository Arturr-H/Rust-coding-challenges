/*- How much is true?
    ----- Create a function which returns the number of true values there are in an array. -*/
pub fn how_much_is_true(input:Vec<bool>) -> usize {
    input.iter().filter(|e| **e).collect::<Vec<&bool>>().len()
}

/*- Seven finder
    Create a function that takes an array of numbers and return
    true if the digit 7 appears in the array. Otherwise, return false. -*/
pub fn find_seven(input:Vec<u32>) -> bool {
    for num in input {
        match num.to_string().contains("7") {
            true => return true,
            false => continue
        };
    };

    false
}

/*- Boomerang finder
    Create a function that returns the total number of boomerangs
    in an array. A boomerang looks like this: [1, 5, 1] or [-2, 8, -2] -*/
pub fn find_boomerangs<'a>(input:Vec<i32>) -> Vec<Vec<i32>> {
    let mut end:Vec<Vec<i32>> = Vec::new();
    for i in 0..input.len()-3 {
        let (c1, c2, c3) = (input[i], input[i+1], input[i+2]);
        if c1 == c3 {
            end.push(vec![c1, c2, c3]);
        };
    };

    end
}

/*- Track the Robot
    This robot roams around a 2D grid. It starts at (0, 0) facing North. After
    each time it moves, the robot rotates 90 degrees clockwise. Given the amount the
    robot has moved each time, you have to calculate the robot's final position. -*/
pub fn track_robot(steps:Vec<i32>) -> (i32, i32) {
    let mut current_pos:(i32, i32) = (0, 0);
    let mut current_dir:u8 = 0;

    for step in steps {
        match current_dir {
            0 => current_pos.0 += step,
            1 => current_pos.1 += step,
            2 => current_pos.0 -= step,
            3 => current_pos.1 -= step,
            _ => panic!("What")
        }

        if current_dir >= 3 { current_dir = 0; }
        else { current_dir += 1; }
    }

    current_pos
}

/*- Zeroes to the End
    Write a function that moves all the zeroes to the end of an array.
    Do this without returning a copy of the input array. -*/
pub fn move_zeroes(input:&mut Vec<i32>) -> () {
    for i in 0..input.len() {
        if input[i] == 0 {
            input.remove(i);
            input.push(0);
        };
    };
}

/*- Two Powers of Two
    Given a number n, return whether it can be expressed as the sum of two powers of two.
    That means the sum of these types of 'doubling' numbers 1, 2, 4, 8, 16, 32, etc ... -*/
pub fn two_powers_of_two(num:u32) -> Option<(u32, u32)> {
    let mut powers:Vec<u32> = vec![1];
    let mut index = 0;

    loop {
        let push = 2u32.pow(index);
        powers.push(push);

        if push > num { break; };
        index += 1;
    };

    for j in &powers {
        for k in &powers {
            if j + k == num { return Some((*j, *k)) };
        };
    };

    None
}

/*- Three Sum Problem
    Write a function that returns all sets of three elements that sum to 0. -*/
pub fn three_sum(input:&Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut fin:Vec<(i32, i32, i32)> = Vec::new();
    for (i_j, j) in input.iter().enumerate() {
        for (i_k, k) in input.iter().enumerate() {
            if i_k == i_j { continue; };
            for (i_a, a) in input.iter().enumerate() {
                if i_a == i_k || i_a == i_j { continue; };
                let mut add = vec![j,k,a];
                add.sort();
                let (n1, n2, n3) = (add[0], add[1], add[2]);
                if j + k + a == 0 && !fin.contains(&(*n1, *n2, *n3)) {
                    fin.push((*n1, *n2, *n3));
                }
            };
        };
    };

    fin
}