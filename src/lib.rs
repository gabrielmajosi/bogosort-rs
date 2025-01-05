use rand::seq::SliceRandom;
use rand::thread_rng;

// Bogosort function
// Randomly shuffle the list until it is properly sorted
// Return resulting list and number of iterations to fully sort
pub fn bogosort(list: Vec<isize>) -> (Vec<isize>, i32) {
    let mut list = list;
    let mut iterations = 0;

    loop {
        list.shuffle(&mut thread_rng());
        iterations += 1;

        if list.is_sorted() {
            break;
        }
    }

    return (list, iterations);
}
