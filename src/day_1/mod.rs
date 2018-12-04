use std::collections::HashSet;

pub fn solve() {
    let frequency_changes: Vec<i64> = aoc::get_input(1)
        .iter()
        .map(|fc| fc.parse().unwrap())
        .collect();

    part_1(&frequency_changes);
    part_2(&frequency_changes);
}

fn part_1(fc: &Vec<i64>) {
    println!("Sum: {}", fc.iter().sum::<i64>());
}

fn part_2(fc: &Vec<i64>) {
    let mut frequencies = HashSet::new();
    let mut frequency = 0;
    let duplicate = fc
        .into_iter()
        .cycle()
        .find_map(|fc| {
            frequency += fc;
            frequencies.replace(frequency)
        }).unwrap();

    println!("Duplicate: {}", duplicate);
}
