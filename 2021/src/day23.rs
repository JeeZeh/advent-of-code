use std::{cmp::Ordering, collections::BinaryHeap};

use ahash::AHashSet;
use itertools::Itertools;

use crate::aocutil::Grid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Amphipod {
    id: usize,
    x: usize,
    y: usize,
    dest_x: usize,
    cost: u32,
    kind: char,
}

impl Amphipod {
    fn new(id: usize, pos: (usize, usize), kind: usize) -> Amphipod {
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
    hall: Vec<(usize, usize)>,
    rooms: Vec<Vec<(usize, usize)>>, // Indexed by [room][slot 0..bottom_y]
}

impl GameState {
    // Returns a 2d array of all the positions indexed [x][y], and the pod
    // kind and id located at that position
    fn generate_locations(&mut self) {
        let mut hall = vec![(usize::MAX, usize::MAX); 12];
        let mut rooms = vec![vec![(usize::MAX, usize::MAX); self.bottom_y + 1]; 12];

        for (kind, pods) in self.pods.iter().enumerate() {
            for (id, pod) in pods.iter().enumerate() {
                if pod.y == self.hall_y {
                    hall[pod.x] = (kind, id);
                } else {
                    rooms[pod.x][pod.y] = (kind, id)
                }
            }
        }
        self.hall = hall;
        self.rooms = rooms;
    }

    fn move_pod(&self, kind: usize, id: usize, dest: (usize, usize), to_room: bool) -> GameState {
        let mut new_state = self.clone();
        let pod = self.pods[kind][id];

        let move_x = (pod.x as i32 - dest.0 as i32).abs() as usize;
        let move_y = (pod.y as i32 - dest.1 as i32).abs() as usize;
        let cost = (move_x + move_y) as u32 * pod.cost;

        new_state.cost += cost;
        new_state.pods[kind as usize][id].x = dest.0 as usize;
        new_state.pods[kind][id].y = dest.1 as usize;

        if to_room {
            new_state.hall[pod.x] = (usize::MAX, usize::MAX);
            new_state.rooms[dest.0][dest.1] = (kind, id);
        } else {
            new_state.rooms[pod.x][pod.y] = (usize::MAX, usize::MAX);
            new_state.hall[dest.0] = (kind, id);
        }

        new_state
    }

    fn try_move_pod(
        &self,
        kind: usize,
        id: usize,
        dest_x: usize,
        to_room: bool,
    ) -> Option<GameState> {
        let pod = self.pods[kind][id];

        // If there's a different kind of pod in our destination room, we can't move in there yet
        if to_room {
            if self.rooms[pod.dest_x]
                .iter()
                .find(|(other_kind, _)| *other_kind != kind && *other_kind != usize::MAX)
                .is_some()
            {
                return None;
            }
        } else {
            // We're in a room, check if we can move out
            let pod_above = self.rooms[pod.x as usize][self.hall_y + 1..pod.y]
                .iter()
                .find(|(k, _)| *k != usize::MAX)
                .is_some();

            if pod_above {
                return None;
            }
        }

        let dest_y;

        if to_room {
            dest_y = (2..=self.bottom_y)
                .rev()
                .find(|y| self.rooms[pod.dest_x][*y].0 == usize::MAX)
                .unwrap();
        } else {
            dest_y = 1;
        }

        // Check that the hallway path is not obstructed
        let walk_dir: i32 = if pod.x < dest_x { 1 } else { -1 };
        let mut walk = pod.x as i32;
        while walk != dest_x as i32 {
            // dbg!(pod.x, dest_x, walk);
            walk += walk_dir;
            if self.hall[walk as usize].0 != usize::MAX {
                return None;
            }
        }

        Some(self.move_pod(kind, id, (dest_x, dest_y), to_room))
    }

    fn is_done(&self) -> bool {
        self.pods.iter().flatten().all(|a| a.x == a.dest_x)
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
    let mut visited: AHashSet<[Vec<Amphipod>; 4]> = AHashSet::new();

    queue.push(start.clone());
    visited.insert(start.pods.clone());

    while let Some(next_state) = queue.pop() {
        if next_state.is_done() {
            return next_state.cost;
        }

        for (kind, pods) in next_state.pods.iter().enumerate() {
            for (id, pod) in pods.iter().enumerate() {
                // Handle some unreachable states
                if pod.x == pod.dest_x {
                    // At the bottom of the destination room = don't move
                    if pod.y == next_state.bottom_y {
                        continue;
                    }
                    // At the top of the destination area, and the same pod type is in
                    // the same room below it (both are in the right spot)
                    if next_state.rooms[pod.x][pod.y + 1].0 == kind {
                        continue;
                    }
                }

                // We're in the hallway
                if pod.y == next_state.hall_y {
                    if let Some(new_state) = next_state.try_move_pod(kind, id, pod.dest_x, true) {
                        if visited.contains(&new_state.pods) {
                            continue;
                        }
                        visited.insert(new_state.pods.clone());
                        queue.push(new_state);
                    }
                } else {
                    // We're in a room and need to move to the hallway
                    // Can't move to doorways (3, 5, 7, 9)
                    // dbg!(&next_state);
                    for dest_x in [1, 2, 4, 6, 8, 10, 11] {
                        if let Some(new_state) = next_state.try_move_pod(kind, id, dest_x, false) {
                            if visited.contains(&new_state.pods) {
                                continue;
                            }
                            visited.insert(new_state.pods.clone());
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
                let pos = ((x as usize * 2) + 3, y as usize + 2);
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
        hall: vec![(usize::MAX, usize::MAX); 12],
        rooms: vec![vec![(usize::MAX, usize::MAX); bottom_y + 1]; 12],
    };
    // Generate initial locations
    state.generate_locations();

    state
}
