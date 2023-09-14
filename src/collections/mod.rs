

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

pub fn iterative_binary_search<T>(vector: &Vec<T>, target: &T) -> Option<usize> where T: PartialEq, T:  PartialOrd {
    let mut high: usize = vector.len();
    let mut low: usize = 0;
    let mut middle: usize;

    for _ in 0..100 {
        
        middle = ((high - low) >> 1) + low;
        if vector[middle] == *target {
            return Some(middle);
        } else if vector[middle] > *target {
            high = middle;
        } else {
            low = middle;
        }
    }

    None
}