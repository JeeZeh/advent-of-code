use std::{cmp::Ordering, collections::BinaryHeap};

use ahash::AHashSet;
use itertools::Itertools;

use crate::aocutil::Grid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Amphipod {
    id: usize,
    x: u8,
    y: u8,
    dest_x: u8,
    cost: u32,
    kind: char,
}

impl Amphipod {
    fn new(id: usize, pos: (u8, u8), kind: u8) -> Amphipod {
        Amphipod {
            id,
            x: pos.0,
            y: pos.1,
            dest_x: (kind * 2) + 3,
            cost: (10 as u32).pow(kind as u32),
            kind: ['A', 'B', 'C', 'D'][kind as usize],
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GameState {
    pods: [Vec<Amphipod>; 4],
    cost: u32,
    hall_y: usize,
    bottom_y: usize,
    locations: Vec<Vec<(usize, usize)>>, // Indexed [x][y], 12*bottom_y+1
}

impl GameState {
    // Returns a 2d array of all the positions indexed [x][y], and the pod
    // kind and id located at that position
    fn generate_locations(&mut self) {
        let mut locations = vec![vec![(usize::MAX, usize::MAX); self.bottom_y + 1]; 12];

        for (kind, pods) in self.pods.iter().enumerate() {
            for (id, pod) in pods.iter().enumerate() {
                locations[pod.x as usize][pod.y as usize] = (kind as usize, id as usize);
            }
        }
        self.locations = locations;
    }

    fn is_slice_obstructed(&self, slice: &[(usize, usize)], start: usize, end: usize) -> bool {
        let walk_dir: i32 = if start < end { 1 } else { -1 };
        let mut walk = start as i32;
        while walk != end as i32 {
            walk += walk_dir;
            if slice[walk as usize].0 != usize::MAX {
                return true;
            }
        }
        false
    }

    fn is_path_obstructed(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let hallway_slice = self
            .locations
            .iter()
            .map(|cols| cols[self.hall_y])
            .collect_vec();

        // dbg!(&hallway_slice);

        // Are we in the hallway?
        if start.1 == self.hall_y {
            // We need to check if we can move to the doorway first
            if self.is_slice_obstructed(&hallway_slice, start.0, end.0) {
                return true;
            }

            // Then we can try move through the room
            let dest_room_slice = &self.locations[end.0];
            if self.is_slice_obstructed(&dest_room_slice, start.1, end.1) {
                return true;
            }
        } else {
            // We need to check if we can move to the hallway first
            let start_room_slice = &self.locations[start.0];
            // dbg!(&start_room_slice, start.0, start.1, end.1);
            if self.is_slice_obstructed(&start_room_slice, start.1, end.1) {
                return true;
            }

            // Then we can try move through the hallway
            if self.is_slice_obstructed(&hallway_slice, start.0, end.0) {
                return true;
            }
        }

        false
    }

    fn move_pod(&self, kind: usize, id: usize, dest: (usize, usize)) -> GameState {
        let mut new_state = self.clone();
        let pod = self.pods[kind][id];

        let move_x = (pod.x as i32 - dest.0 as i32).abs() as u8;
        let move_y = (pod.y as i32 - dest.1 as i32).abs() as u8;
        let cost = (move_x + move_y) as u32 * pod.cost;
        new_state.cost += cost;
        new_state.pods[kind as usize][id].x = dest.0 as u8;
        new_state.pods[kind][id].y = dest.1 as u8;
        new_state.generate_locations();

        new_state
    }

    fn try_move_pod_to_destination(&self, kind: usize, id: usize) -> Option<GameState> {
        let pod = self.pods[kind][id];

        // If there's a different kind of pod in our destination room, we can't move in there yet
        if self.locations[pod.dest_x as usize]
            .iter()
            .find(|(other_kind, _)| *other_kind != kind && *other_kind != usize::MAX)
            .is_some()
        {
            return None;
        }

        let dest_y = (2..=self.bottom_y)
            .rev()
            .find(|y| self.locations[pod.dest_x as usize][*y].0 == usize::MAX)
            .unwrap();

        // Check that the hallway path is not obstructed
        if self.is_path_obstructed(
            (pod.x as usize, pod.y as usize),
            (pod.dest_x as usize, dest_y),
        ) {
            return None;
        }

        Some(self.move_pod(kind, id, (pod.dest_x as usize, dest_y)))
    }

    fn try_move_pod_to_hallway_x(&self, kind: usize, id: usize, x: usize) -> Option<GameState> {
        let pod = self.pods[kind][id];
        let dest_y: usize = 1;

        // dbg!(&pod, x);
        // let pod_above = self.locations[pod.x as usize][self.hall_y + 1..pod.y as usize]
        //     .iter()
        //     .find(|(k, _)| *k != usize::MAX);

        // If we're not the top-most pod in the room and there's a pod above us,
        // we can't move yet.
        if self.is_path_obstructed((pod.x as usize, pod.y as usize), (x, dest_y)) {
            return None;
        }

        Some(self.move_pod(kind, id, (x as usize, dest_y)))
    }

    fn is_done(&self) -> bool {
        self.pods.iter().flatten().all(|a| a.x == a.dest_x)
    }

    fn print(&self) {
        let mut lines = vec![vec![' '; self.locations.len()]; self.locations[0].len()];

        for (x, col) in self.locations.iter().enumerate() {
            for (y, row) in col.iter().enumerate() {
                if row.0 != usize::MAX {
                    lines[y][x] = self.pods[row.0][row.1].kind;
                }
            }
        }
        lines.show_display();
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Terribly adapted from
// https://www.reddit.com/r/adventofcode/comments/rmnozs/comment/hpnoqsj/?utm_source=share&utm_medium=web2x&context=3
pub fn solve(mut lines: Vec<String>) -> (u32, u32) {
    let starting_state = parse_game_state(&lines);
    let part_one = move_pods(&starting_state);

    lines.insert(3, String::from("  #D#C#B#A#"));
    lines.insert(4, String::from("  #D#B#A#C#"));

    let extended_state = parse_game_state(&lines);
    let part_two = move_pods(&extended_state);

    (part_one, part_two)
}

fn move_pods(start: &GameState) -> u32 {
    // Dijkstra
    let mut queue: BinaryHeap<GameState> = BinaryHeap::new();
    let mut visited: AHashSet<GameState> = AHashSet::new();

    queue.push(start.clone());
    visited.insert(start.clone());

    while let Some(next_state) = queue.pop() {
        if next_state.is_done() {
            next_state.print();
            return next_state.cost;
        }

        for (kind, pods) in next_state.pods.iter().enumerate() {
            for (id, pod) in pods.iter().enumerate() {
                // Handle some unreachable states
                if pod.x == pod.dest_x {
                    // At the bottom of the destination room = don't move
                    if pod.y == 3 {
                        continue;
                    }
                    // At the top of the destination area, and the same pod type is in
                    // the same room below it (both are in the right spot)
                    if next_state.locations[pod.x as usize][next_state.bottom_y].0 == kind {
                        continue;
                    }
                }

                // We're in the hallway
                if pod.y == 1 {
                    if let Some(new_state) = next_state.try_move_pod_to_destination(kind, id) {
                        if visited.contains(&new_state) {
                            continue;
                        }
                        visited.insert(new_state.clone());
                        queue.push(new_state);
                    }
                } else {
                    // We're in a room and need to move to the hallway
                    // Can't move to doorways (3, 5, 7, 9)
                    // dbg!(&next_state);
                    for dest_x in [1, 2, 4, 6, 8, 10, 11] {
                        if let Some(new_state) =
                            next_state.try_move_pod_to_hallway_x(kind, id, dest_x)
                        {
                            if visited.contains(&new_state) {
                                continue;
                            }
                            visited.insert(new_state.clone());
                            queue.push(new_state);
                        }
                    }
                }
            }
        }
    }

    unreachable!("We should have found a finished state!");
}

fn parse_game_state(lines: &Vec<String>) -> GameState {
    let mut parsed_pods: [Vec<Amphipod>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for (y, line) in lines.iter().skip(2).enumerate() {
        let pods = line.split('#').collect_vec();
        let pod_idxs = if pods.len() == 6 {
            [1, 2, 3, 4]
        } else {
            [3, 4, 5, 6]
        };

        if pods.len() == 6 || (pods.len() == 10 && !pods.contains(&"  ")) {
            for (x, pod_idx) in pod_idxs.iter().enumerate() {
                let pos = ((x as u8 * 2) + 3, y as u8 + 2);
                let (type_, parsed) = match pods[*pod_idx as usize] {
                    "A" => (0, Amphipod::new(parsed_pods[0].len(), pos, 0)),
                    "B" => (1, Amphipod::new(parsed_pods[1].len(), pos, 1)),
                    "C" => (2, Amphipod::new(parsed_pods[2].len(), pos, 2)),
                    "D" => (3, Amphipod::new(parsed_pods[3].len(), pos, 3)),
                    _ => panic!("Unexpected pod type"),
                };

                parsed_pods[type_].push(parsed);
            }
        }
    }

    let bottom_y = lines.len() - 2;

    let mut state = GameState {
        pods: parsed_pods,
        hall_y: 1,
        bottom_y,
        cost: 0,
        locations: vec![vec![(usize::MAX, usize::MAX); bottom_y + 3]; 12],
    };
    // Generate initial locations
    state.generate_locations();

    state
}
