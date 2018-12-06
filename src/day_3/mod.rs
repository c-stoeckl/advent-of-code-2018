use std::collections::HashMap;

struct Rectangle {
    id: u64,
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

impl Rectangle {
    fn get_coordinates(&self) -> Vec<(u64, u64)> {
        let mut coordinates = Vec::new();

        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                coordinates.push((x, y));
            }
        }

        return coordinates;
    }
}

pub fn solve() {
    let claims: Vec<String> = aoc::get_input(3);
    let rectangles = parse_claims(claims);
    let fabric = generate_fabric(&rectangles);
    let overlaps = fabric.values().filter(|&&v| v >= 2).count();

    println!("[I] Overlapping square inches of fabric: {}", overlaps);

    for rectangle in rectangles {
        if !is_overlapping(&rectangle, &fabric) {
            println!("[II] Claim-ID with no overlap: #{}", rectangle.id);
        }
    }
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

fn generate_fabric(rectangles: &Vec<Rectangle>) -> HashMap<(u64, u64), u64> {
    let mut fabric = HashMap::new();

    for rectangle in rectangles {
        rectangle.get_coordinates().iter().for_each(|&x| {
            *fabric.entry(x).or_insert(0) += 1;
        });
    }

    return fabric;
}

fn is_overlapping(rectangle: &Rectangle, fabric: &HashMap<(u64, u64), u64>) -> bool {
    rectangle
        .get_coordinates()
        .iter()
        .any(|x| fabric.get(x).unwrap() != &1)
}
