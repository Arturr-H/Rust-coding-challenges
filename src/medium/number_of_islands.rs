/// Given an m x n 2D binary grid grid which represents a map of 1s (land) and 0s (water), return
/// the number of islands. An island is surrounded by water and is formed by connecting adjacent lands
/// horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
pub fn run(mut matrix:Vec<Vec<u8>>) -> u32 {
    let mut total_islands:u32 = 0;
    for (y, row) in matrix.clone().iter().enumerate() {
        for x in 0..row.len() {
            if matrix[y][x] == 1 { remove_island(x, y, &mut matrix); total_islands += 1; }
        }
    }
    
    total_islands
}
fn remove_island(x:usize, y:usize, matrix:&mut Vec<Vec<u8>>) -> Option<()> {
    if x > matrix.len() || y > matrix.len()-1 || matrix[y][x] == 0 { return None };
        matrix[y][x] = 0;

    remove_island(x + 1, y, matrix);
    match x.checked_sub(1) { Some(e) => remove_island(e, y, matrix), None => None };
    remove_island(x, y + 1, matrix);
    match y.checked_sub(1) { Some(e) => remove_island(x, e, matrix), None => None };
    Some(())
}