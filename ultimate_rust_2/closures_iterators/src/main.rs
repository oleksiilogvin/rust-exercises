fn main() {
    let square = |x: i32| x * x;
    println!("5 squared is {}", square(5));

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    let pairs_for = pairs.clone();
    for pair in pairs_for {
        println!("{:?}", (pair.0 + 1, pair.1));
    }
    pairs
        .into_iter()
        .map(|(x, y)| (x + 1, y))
        .for_each(|t| println!("{:?}", t));

    let mut numbers = vec![1, 2, 3, 4];
    let mut mut_numbers_func = numbers.clone();
    for x in &mut numbers {
        *x *= 3;
    }
    println!("{:?}", numbers);
    mut_numbers_func.iter_mut().for_each(|x| *x *= 3);
    println!("{:?}", mut_numbers_func);

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed: Vec<String> = words
        .into_iter()
        .filter(|word| !word.contains('h'))
        .map(|word| word.to_uppercase())
        .collect();
    println!("Transformed: {:?}", transformed);
}
