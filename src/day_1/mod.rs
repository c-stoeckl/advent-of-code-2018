use std::collections::HashSet;

pub fn solve() {
    let frequency_changes: Vec<i64> = aoc::get_input(1)
        .iter()
        .map(|fc| fc.parse().unwrap())
        .collect();

    println!("Sum: {}", frequency_changes.iter().sum::<i64>());

    let mut frequencies = HashSet::new();
    let mut frequency = 0;
    let duplicate = frequency_changes
        .into_iter()
        .cycle()
        .find_map(|fc| {
            frequency += fc;
            frequencies.replace(frequency)
        }).unwrap();

    println!("Duplicate: {}", duplicate);
}
