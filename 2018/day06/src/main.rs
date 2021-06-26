use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

#[derive(Eq, Hash)]
struct Pos(i32, i32);
struct Landing(i8, Pos);

#[derive(Clone, Debug)]
struct Nearest(i8, usize);

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn main() {
    let landings: Vec<Landing> = fs::read_to_string("./src/input")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, l)| Landing(i as i8, parse_pos(l)))
        .collect();

    let grid = generate_grid(&landings);
    let (largest_finite, largest_under_10000) = largest_finite(&grid);

    println!(
        "Largest finite area: {}\nLargest area < 10000 steps from all landing spots: {}",
        largest_finite, largest_under_10000
    )
}

fn largest_finite(grid: &HashMap<Pos, (Nearest, usize)>) -> (usize, usize) {
    let mut counts: HashMap<i8, usize> = HashMap::new();
    let mut infinite: HashSet<i8> = HashSet::new();
    let mut area_under_10000 = 0;
    infinite.insert(-1);

    for (pos, (nearest, total_distances)) in grid {
        if pos.0.abs() == 1000 || pos.1.abs() == 1000 {
            infinite.insert(nearest.0);
        }

        if total_distances < &10_000 {
            area_under_10000 += 1;
        }

        let cref = counts.entry(nearest.0).or_insert(0);
        *cref += 1;
    }

    return (
        *counts
            .iter()
            .filter(|(id, _)| !infinite.contains(id))
            .map(|(_, count)| count)
            .max()
            .unwrap(),
        area_under_10000,
    );
}

fn generate_grid(landings: &Vec<Landing>) -> HashMap<Pos, (Nearest, usize)> {
    let mut grid: HashMap<Pos, (Nearest, usize)> = HashMap::new();

    for y in -1000..=1000 {
        for x in -1000..=1000 {
            let distances: Vec<Nearest> = landings
                .iter()
                .map(|landing| Nearest(landing.0, get_manhattan(&landing.1, &Pos(x, y))))
                .collect();

            let mut nearest = distances
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|s| Nearest(s.0, s.1))
                .unwrap();

            // If more than one smallest distance (they're tied) reset Nearest to id -1 (meaningless, ".")
            if distances.iter().filter(|n| n.1 == nearest.1).count() > 1 {
                nearest = Nearest(-1, 0);
            }

            grid.insert(Pos(x, y), (nearest, distances.iter().map(|n| n.1).sum()));
        }
    }

    return grid;
}

fn get_manhattan(a: &Pos, b: &Pos) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn parse_pos(line: &str) -> Pos {
    let mut split = line.split(", ");
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();

    Pos(x, y)
}
