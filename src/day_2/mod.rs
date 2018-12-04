pub fn solve() {
    let box_ids: Vec<String> = aoc::get_input(2);

    doubles_triples(&box_ids);
    compare_strings(&box_ids);
}

fn compare_strings(box_ids: &Vec<String>) {
    let mut box_ids_checked: Vec<&str> = vec![];
    let mut result = String::new();

    for box_id in box_ids {
        for box_id_target in box_ids {
            if box_id == box_id_target || box_ids_checked.iter().any(|x| x == box_id_target) {
                continue;
            }

            let mut index = 0;
            let mut diff_count = 0;
            while index < box_id.len() {
                if box_id.chars().nth(index) != box_id_target.chars().nth(index) {
                    diff_count = diff_count + 1;
                }
                index = index + 1;
            }

            if diff_count == 1 {
                result = box_id
                    .chars()
                    .into_iter()
                    .zip(box_id_target.chars())
                    .filter(|t| t.0 == t.1)
                    .map(|t| t.0)
                    .collect();
            }
        }

        box_ids_checked.push(box_id);
    }

    println!("Common letters: {}", result);
}

fn doubles_triples(box_ids: &Vec<String>) {
    let mut doubles_sum: i32 = 0;
    let mut triples_sum: i32 = 0;

    for box_id in box_ids {
        let mut chars_checked: Vec<char> = vec![];
        let mut has_double: bool = false;
        let mut has_triple: bool = false;

        for c in box_id.chars() {
            let mut char_count: i32 = 0;

            if chars_checked.iter().any(|cc| cc == &c) {
                continue;
            }

            for c2 in box_id.chars() {
                if c == c2 {
                    char_count = char_count + 1;
                }
            }

            chars_checked.push(c);

            if char_count == 2 {
                has_double = true;
            } else if char_count == 3 {
                has_triple = true;
            }
        }

        if has_double {
            doubles_sum = doubles_sum + 1;
        }

        if has_triple {
            triples_sum = triples_sum + 1;
        }
    }

    println!("Checksum: {}", doubles_sum * triples_sum);
}
