use std::collections::HashMap;

struct Rectangle {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

pub fn solve() {
    let claims: Vec<String> = aoc::get_input(3);
    let rectangles = parse_claims(claims);
    let fabric = generate_area(rectangles);
    let overlaps = fabric.values().filter(|&&v| v >= 2).count();

    println!("[I] Overlaps: {}", overlaps);
}

fn parse_claims(claims: Vec<String>) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();

    for claim in claims {
        let slices: Vec<&str> = claim.split(" ").collect();
        let positions: Vec<u64> = slices[2]
            .trim_end_matches(":")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let dimensions: Vec<u64> = slices[3].split("x").map(|s| s.parse().unwrap()).collect();

        let id: u64 = slices[0].trim_start_matches("#").parse().unwrap();
        let (x, y) = (positions[0], positions[1]);
        let (width, height) = (dimensions[0], dimensions[1]);

        rectangles.push(Rectangle {
            id,
            x,
            y,
            width,
            height,
        })
    }

    return rectangles;
}

fn generate_area(rectangles: Vec<Rectangle>) -> HashMap<(u64, u64), u64> {
    let mut fabric = HashMap::new();

    for rectangle in rectangles {
        for y in rectangle.y..(rectangle.y + rectangle.height) {
            for x in rectangle.x..(rectangle.x + rectangle.width) {
                let position = (x, y);
                *fabric.entry(position).or_insert(0) += 1;
            }
        }
    }

    return fabric;
}
