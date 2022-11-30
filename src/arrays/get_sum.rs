/// Get an array of integers and an integer sum and check if there are any combination in the array that sums to sum
pub mod run {
    pub fn nested(array:&[i32], sum:i32) -> bool {
        for (i, el) in array.iter().enumerate() {
            for j in i+1..array.len() {
                let next = array[j];
                
                if el + next == sum { return true; };
            }
        }
        
        false
    }
    pub fn backchecked(array:&[i32], sum:i32) -> bool {
        let mut prev:Vec<i32> = Vec::with_capacity(array.len());

        for el in array {
            for item in &prev {
                if el == item { return true; };
            };

            prev.push(sum - el);
        };

        false
    }
}
