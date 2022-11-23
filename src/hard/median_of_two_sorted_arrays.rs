/// Given two sorted arrays nums1 and nums2 of size m and n
/// respectively, return the median of the two sorted arrays.
pub fn run(arr1:&[u8], arr2:&[u8]) -> f32 {
    let mut arr = vec![arr1, arr2].concat();
    arr.sort();
    let len = arr.len();
    if len % 2 == 0 {
        return (arr[len/2 - 1] as f32 + arr[len/2] as f32) / 2.
    }else {
        return arr[len/2] as f32
    }
}
fn run_inner(arr1:&[u8], arr2:&[u8]) -> f32 {
    let mut arr = vec![arr1, arr2].concat();
    arr.sort();
    let len = arr.len();
    if len % 2 == 0 {
        return (arr[len/2 - 1] as f32 + arr[len/2] as f32) / 2.
    }else {
        return arr[len/2] as f32
    }
}