use std::cmp::Ordering;

pub fn ceil_binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left:i64 = 0;
    let mut right:i64 = arr.len() as i64-1;
    while left <= right {
        let mid = left + ((right - left) >> 1);       
        match item.cmp(&arr[mid as usize]) {
            Ordering::Less => right = mid - 1,
            Ordering::Equal => return Some(mid as usize),
            Ordering::Greater => left = mid + 1,
        }      
    }
    Some(left as usize)
}