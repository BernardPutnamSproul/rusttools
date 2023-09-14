

pub fn binary_search<T>(vector: &Vec<T>, target: &T) -> Option<usize> where T: PartialEq, T: PartialOrd {
    internal_search(vector, target, 0, vector.len())
} 

fn internal_search<T>(vector: &Vec<T>, target: &T, low: usize, high: usize) -> Option<usize> where T: PartialEq, T: PartialOrd {
    let middle: usize = ((high - low) >> 1) + low;
    
    if low > middle || middle >= vector.len() {
        None
    } else if vector[middle] == *target {
        Some(middle)
    } else if vector[middle] > *target {
        internal_search(vector, target, low, middle - 1)
    } else {
        internal_search(vector, target, middle + 1, high)
    }
}

pub fn iterative_binary_search<T>(vector: &Vec<T>, target: &T) -> Option<usize> where T: PartialEq, T:  PartialOrd {
    let mut high: usize = vector.len();
    let mut low: usize = 0;
    let mut middle: usize;

    loop {
        middle = ((high - low) >> 1) + low;
        
        if low > middle || middle >= vector.len() {
            break None;
        }
        
        if vector[middle] == *target {
            break Some(middle);
        } else if vector[middle] > *target {
            high = middle - 1;
        } else {
            low = middle + 1;
        }
    }
}