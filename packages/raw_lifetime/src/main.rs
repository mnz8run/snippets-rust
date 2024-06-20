fn main() {
    // let array1 = vec![1, 2, 3];
    let array1 = vec![];
    let array2 = vec![4, 5, 6];

    let result = first_of_two(&array1, &array2);
    println!("The first element is: {}", result);
}

// lifetime parameters must be declared prior to type and const parameters
fn first_of_two<'a, T>(slice1: &'a [T], slice2: &'a [T]) -> &'a T {
    if !slice1.is_empty() {
        &slice1[0]
    } else if !slice2.is_empty() {
        &slice2[0]
    } else {
        panic!("Both slices are empty!")
    }
}
