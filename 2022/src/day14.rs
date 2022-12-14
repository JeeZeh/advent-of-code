use itertools::Itertools;

use crate::aocutil::Grid;

pub fn solve(input: String) -> (usize, usize) {
    let (mut map, max_x, max_y) = build_map(&input);
    let mut abyss_time = 0;
    let source = Pos(500, 0);
    while spawn_sand(&mut map, &source).is_some() {
        abyss_time += 1;
    }

    let mut source_time = abyss_time;
    for x in 0..max_x * 2 {
        map[max_y + 2][x] = State::Rock;
    }
    while let Some(landed) = spawn_sand(&mut map, &source) {
        source_time += 1;
        if landed == source {
            break;
        };
    }
    // show_map(&map, Pos(490, 0), Pos(510, 30));

    (abyss_time, source_time)
}

// Create a unit of sand at a position, and let it fall to a resting point, or t̸h̵e̵ ̴v̵o̶i̴d̷
fn spawn_sand(map: &mut Vec<Vec<State>>, from: &Pos) -> Option<Pos> {
    if let Some(landed) = find_resting_point(map, from) {
        map[landed.1][landed.0] = State::Sand;
        return Some(landed);
    }
    return None;
}

// Free-fall downwards until we come in contact with t̸h̵e̵ ̴v̵o̶i̴d̷... or sand/rock.
fn try_fall(map: &Vec<Vec<State>>, from: &Pos) -> Option<Pos> {
    let down = from.down();
    if let Some(p) = map.getyx(down.1, down.0) {
        if *p == State::Air {
            return Some(down);
        };
    }
    return None;
}

// From a starting point, find where it will land recursively; if it can land somewhere.
fn find_resting_point(map: &Vec<Vec<State>>, from: &Pos) -> Option<Pos> {
    let mut current = *from;
    while let Some(next) = try_fall(map, &current) {
        current = next;
    }

    let down = current.down();
    match map.getyx(down.1, down.0) {
        Some(State::Rock | State::Sand) => {
            // Check diagonally left.
            let down_left = current.down_left();
            match map.getyx(down_left.1, down_left.0) {
                // Found a space!
                Some(State::Air) => return find_resting_point(map, &down_left),
                _ => (),
            }
            // Check diagonally right.
            let down_right = current.down_right();
            match map.getyx(down_right.1, down_right.0) {
                Some(State::Air) => return find_resting_point(map, &down_right),
                _ => (),
            }
            // Can't go anywhere, so we're resting here.
            return Some(current);
        }
        Some(x) => panic!("Didn't expect {:?}", *x),
        None => return None, // t̸h̵e̵ ̴v̵o̶i̴d̷
    };
}

fn build_map(input: &str) -> (Vec<Vec<State>>, usize, usize) {
    let paths = input
        .lines()
        .map(|l| l.split(" -> ").map(Pos::from).collect_vec())
        .collect_vec();

    let max_x = paths.iter().flatten().max_by_key(|p| p.0).unwrap().0;
    let max_y = paths.iter().flatten().max_by_key(|p| p.1).unwrap().1;
    let mut map = vec![vec![State::Air; max_x * 2]; max_y + 3];

    paths.iter().for_each(|p| trace_path(&mut map, p));
    map[0][500] = State::Source;
    (map, max_x, max_y)
}

// Trace a path on the map, starting from lowest x or y to highest x or y inclusive.
fn trace_path(map: &mut Vec<Vec<State>>, path: &[Pos]) {
    for window in path.windows(2) {
        let Pos(start_x, start_y) = window[0];
        let Pos(end_x, end_y) = window[1];
        if start_y == end_y {
            let start = start_x.min(end_x);
            let end = start_x.max(end_x);
            (start..=end).for_each(|x| {
                map[start_y][x] = State::Rock;
            });
        }
        let start = start_y.min(end_y);
        let end = start_y.max(end_y);
        (start..=end).for_each(|y| {
            map[y][start_x] = State::Rock;
        });
    }
}

fn show_map(map: &Vec<Vec<State>>, Pos(from_x, from_y): Pos, Pos(to_x, to_y): Pos) {
    let w = (to_x - from_x) + 1;
    let h = (to_y - from_y) + 1;
    let mut grid_to_show: Vec<Vec<char>> = vec![vec![State::Air.into(); w]; dbg!(h)];
    for y in from_y..=to_y {
        for x in from_x..=to_x {
            if let Some(c) = map.getyx(y, x) {
                grid_to_show[y - from_y][x - from_x] = (*c).into();
            } else {
                grid_to_show[y - from_y][x - from_x] = ' ';
            };
        }
    }
    grid_to_show.show_display();
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    Rock,
    Air,
    Sand,
    Source,
}

impl From<State> for char {
    fn from(s: State) -> Self {
        match s {
            State::Rock => '#',
            State::Air => '.',
            State::Sand => 'o',
            // State::FlowingSand => '~',
            State::Source => '+',
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pos(usize, usize);

impl Pos {
    fn down(&self) -> Self {
        Pos(self.0, self.1 + 1)
    }
    fn down_left(&self) -> Self {
        Pos(self.0 - 1, self.1 + 1)
    }
    fn down_right(&self) -> Self {
        Pos(self.0 + 1, self.1 + 1)
    }
}

impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Pos(x.parse().unwrap(), y.parse().unwrap())
    }
}
