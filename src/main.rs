fn main() {
    println!("Bogosorter");

    let input_list = (0..5).collect::<Vec<isize>>();
    println!("Input list: {:?}", input_list);

    let sorted = bogosorter::bogosort(input_list);
    println!("Sorted list: {:?}, shuffled {} times", sorted.0, sorted.1);
}
