

pub fn binary_search<T>(vector: &Vec<T>, target: &T) -> Option<usize> where T: PartialEq, T: PartialOrd {
    internal_search(vector, target, 0, vector.len(), 0)
} 

fn internal_search<T>(vector: &Vec<T>, target: &T, low: usize, high: usize, iteration: u32) -> Option<usize> where T: PartialEq, T: PartialOrd {
    let middle: usize = ((high - low) >> 1) + low;
    if vector[middle] == *target {
        Some(middle)
    } else if low > high || iteration > 100 {
        None
    } else if vector[middle] > *target {
        internal_search(vector, target, low, middle, iteration + 1)
    } else {
        internal_search(vector, target, middle, high, iteration + 1)
    }
}