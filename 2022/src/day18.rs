use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

const DIRECTIONS: [Pos; 6] = [
    Pos(-1, 0, 0), // Left
    Pos(1, 0, 0),  // Right
    Pos(0, 1, 0),  // Up
    Pos(0, -1, 0), // Down
    Pos(0, 0, 1),  // Forward
    Pos(0, 0, -1), // Back
];

pub fn solve(input: String) -> (usize, usize) {
    let mut map: HashMap<Pos, bool> = HashMap::new();
    input.lines().map(Pos::from).for_each(|p| {
        map.insert(p, true);
    });

    (count_exposed_sides(&map), walk_exterior(&map))
}

/// This is essentially a flood-fill of the exterior
/// which counts every time we see a face of a cube of the droplet
fn walk_exterior(map: &HashMap<Pos, bool>) -> usize {
    // Keep track of exposed cube faces encountered.
    // Since we're using BFS it guarantees that we will only see a
    // given face once, so we don't need to track it on a per-cube level.
    let mut exterior_faces = 0;

    // Start somewhere outside the droplet
    let (min_x, min_y, min_z, max_x, max_y, max_z) = get_bounds(map);
    let start = Pos(min_x - 1, min_y - 1, min_z - 1);

    // Set up our BFS
    let mut seen: HashSet<Pos> = HashSet::new();
    seen.insert(start.clone());
    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        // Look in every direction for either:
        // - a cube, in which case we've found an exposed face
        // - an empty space, which might be or lead to another exposed face
        for delta in DIRECTIONS {
            let check = &current + &delta;
            // We're bordering a cube, and guaranteed that this is the only
            // time we'll see it from this side, so increment the counter
            if map.contains_key(&check) {
                exterior_faces += 1;
            } else if !seen.contains(&check)
                // Don't allow exploration past the boundaries
                // since we're working with a hashmap
                && min_x - 1 <= check.0
                && min_y - 1 <= check.1
                && min_z - 1 <= check.2
                && check.0 <= max_x + 1
                && check.1 <= max_y + 1
                && check.2 <= max_z + 1
            {
                queue.push_back(check.clone());
                seen.insert(check);
            }
        }
    }
    exterior_faces
}

/// Just look around each cube to find empty spaces
fn count_exposed_sides(map: &HashMap<Pos, bool>) -> usize {
    let mut sides = 0;
    for cube in map.keys() {
        for delta in DIRECTIONS {
            if !map.contains_key(&(cube + &delta)) {
                sides += 1;
            }
        }
    }
    sides
}

fn get_bounds(map: &HashMap<Pos, bool>) -> (i32, i32, i32, i32, i32, i32) {
    let min_x = map.keys().map(|p| p.0).min().unwrap();
    let min_y = map.keys().map(|p| p.1).min().unwrap();
    let min_z = map.keys().map(|p| p.2).min().unwrap();
    let max_x = map.keys().map(|p| p.0).max().unwrap();
    let max_y = map.keys().map(|p| p.1).max().unwrap();
    let max_z = map.keys().map(|p| p.2).max().unwrap();

    (min_x, min_y, min_z, max_x, max_y, max_z)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos(i32, i32, i32);

impl Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',').map(|p| p.parse::<i32>().unwrap());

        Pos(
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        )
    }
}
