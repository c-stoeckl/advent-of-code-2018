use std::collections::HashMap;

extern crate chrono;
use self::chrono::Timelike;

/*
This is very unfinished and needs to be refactored.
Solutions was only found by printing and manually
searching for the right value.
*/

pub fn solve() {
    let records_unsorted: Vec<String> = aoc::get_input(4);
    let records = sort_records(records_unsorted).into_iter();
    let mut guards: HashMap<u64, HashMap<u32, u8>> = HashMap::new();
    let mut guard_id: u64 = 0;
    let mut start_minute: u32 = 0;
    let mut end_minute: u32;

    for record in records {
        let (time, action) = record;
        let action = action.split(" ").collect::<Vec<&str>>();
        let last_word = action.last().unwrap();

        if last_word == &"shift" {
            guard_id = action[1].trim_start_matches('#').parse().unwrap();
        } else if last_word == &"asleep" {
            start_minute = time.minute();
        } else if last_word == &"up" {
            end_minute = time.minute();

            let mut temp_hash = HashMap::new();
            (start_minute..end_minute).for_each(|x| {
                *temp_hash.entry(x).or_insert(0) += 1;
            });

            let sleep = guards.entry(guard_id).or_insert(temp_hash.clone());

            temp_hash
                .into_iter()
                .for_each(|(k, v)| *sleep.entry(k).or_insert(v) += 1)
        }
    }

    for (guard, sleep) in guards {
        // let sleep_sum: u64 = sleep.values().map(|x| *x as u64).sum();
        // let fav_min = sleep.iter().max_by(|x, y| x.iter().max().cmp(y.iter().max())).unwrap();
        let fav_min = sleep.values().max().unwrap();
        println!("Guard #{:?} spends the most minutes asleep at {}", guard, sleep.values().max().unwrap());
    }

    // println!("{:?}", guards.get(&1307).unwrap());
}

fn sort_records(records_unsorted: Vec<String>) -> Vec<(chrono::NaiveDateTime, String)> {
    let mut records = Vec::new();

    for record in records_unsorted {
        let (date, rest) = record.split_at(18);
        let date = chrono::NaiveDateTime::parse_from_str(date, "[%Y-%m-%d %H:%M]").unwrap();
        records.push((date, rest.trim().to_string()));
    }

    records.sort_by(|a, b| a.0.cmp(&b.0));

    return records;
}
