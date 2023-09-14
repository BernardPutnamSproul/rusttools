use tools::collections;


const MAX: i32 = 10000;


#[test]
fn binary_search_test() {
    let mut vector: Vec<i32> = Vec::new();

    for i in -MAX..=MAX {
        vector.push(i);
    }
    

    for i in -MAX..=MAX {
        assert_eq!((i + MAX) as usize, collections::binary_search(&vector, &i).unwrap());
        assert_eq!((i + MAX) as usize, collections::iterative_binary_search(&vector, &i).unwrap());
    }

    assert_eq!(None, collections::binary_search(&vector, &(MAX + 1)));
    assert_eq!(None, collections::iterative_binary_search(&vector, &(MAX + 1)));
}