/// You are given an array of integers distance. You start at point (0,0) on an
/// X-Y plane and you move distance[0] meters to the north, then distance[1] meters
/// to the west, distance[2] meters to the south, distance[3] meters to the east, and
/// so on.
/// 
/// In other words, after each move, your direction changes counter-clockwise. 
/// Return true if your path crosses itself, and false if it does not.
pub fn run(distances:&[u32]) -> bool {
    for i in 3..distances.len() {
        if i >= 3 && distances[i] >= distances[i - 2] && distances[i - 1] <= distances[i - 3] { return true };
        
        if i >= 4 && distances[i - 1] == distances[i - 3] && distances[i] + distances[i - 4] >= distances[i - 2] { return true };
        
        if i >= 5 && distances[i - 2] >= distances[i - 4] && distances[i - 5] + distances[i - 1] >= distances[i - 3] && distances[i - 1] <= distances[i - 3] && distances[i - 4] + distances[i] >= distances[i - 2] { return true };
    }
    
    return false;
}
