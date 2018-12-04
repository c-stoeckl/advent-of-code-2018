use std::fs::File;
use std::io::prelude::*;

pub fn get_input(day: u8) -> Vec<String> {
    let filename = format!("day_{}.txt", day);
    let mut f = File::open(filename).expect("File not found!");

    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    return lines;
}

pub fn clear() {
    std::io::stdout()
        .write_all("\x1b[2J\x1b[1;1H".as_bytes())
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
