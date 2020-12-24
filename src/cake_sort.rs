// unused but still cool <3

// merge sort for sorting cake tuple vector
pub fn merge_sort(list: &mut Vec<(u32, u32)>) { // we *want* to modify, but not own, the source list
    recursive_split(&mut list.clone(), list, 0, list.len()); // "list" here is already a mutable reference ...
    // ... that's why some function paramaters are passed as-is, not with extra "&mut": it's already &mut (i.e. already "just a mutable reference to the data")
    println!("Sorted list: {:?}", list);
}

// part of merge sort
fn recursive_split(alpha: &mut Vec<(u32, u32)>, beta: &mut Vec<(u32, u32)>, start: usize, end: usize) {
    if end - start <= 1 {
        return; // base condition
    }
    let split: usize = (start + end) / 2;
    recursive_split(beta, alpha, start, split);
    recursive_split(beta, alpha, split, end);
    chain_merge(alpha, beta, start, split, end);
}

// part of merge sort; sorts cakes by value density (.1/.0, i.e. value-per-unit-weight)
fn chain_merge(src: &mut Vec<(u32, u32)>, dest: &mut Vec<(u32, u32)>, start: usize, split: usize, end: usize) {
    let mut left = start;
    let mut right = split;
    let mut write = start;
    while write < end {
        if left < split && (right >= end || (f64::from(src[left].1) / (f64::from(src[left].0)) <= (f64::from(src[right].1) / f64::from(src[right].0)))) {
            dest[write] = src[left];
            left += 1;
        } else {
            dest[write] = src[right];
            right += 1;
        }
        write += 1;
    }
}