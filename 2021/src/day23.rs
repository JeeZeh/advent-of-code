use std::{cmp::Ordering, collections::BinaryHeap};

use ahash::{AHashMap, AHashSet};
use itertools::Itertools;

static POD_DEST: [usize; 4] = [3, 5, 7, 9];
static POD_COST: [u32; 4] = [1, 10, 100, 1000];

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GameState {
    pods: [[(usize, usize); 4]; 4],
    cost: u32,
}

impl GameState {
    // Returns a 2d array of all the positions indexed [x][y], and the pod
    // kind and id located at that position
    fn generate_locations(
        &self,
        species_count: usize,
    ) -> ([(usize, usize); 12], [[(usize, usize); 7]; 12]) {
        let mut hall: [(usize, usize); 12] = [(usize::MAX, usize::MAX); 12];
        let mut rooms = [[(usize::MAX, usize::MAX); 7]; 12];

        for (kind, pods) in self.pods.iter().enumerate() {
            for (id, (x, y)) in pods[..species_count].iter().enumerate() {
                if *y == 1 {
                    hall[*x] = (kind, id);
                } else {
                    rooms[*x][*y] = (kind, id)
                }
            }
        }
        (hall, rooms)
    }

    fn move_pod(&self, kind: usize, id: usize, dest: (usize, usize)) -> GameState {
        let mut new_state = self.clone();
        let (x, y) = self.pods[kind][id];

        let move_x = (x as i32 - dest.0 as i32).abs() as usize;
        let move_y = (y as i32 - dest.1 as i32).abs() as usize;
        let cost = (move_x + move_y) as u32 * POD_COST[kind];

        new_state.cost += cost;
        new_state.pods[kind as usize][id].0 = dest.0 as usize;
        new_state.pods[kind][id].1 = dest.1 as usize;

        new_state
    }

    fn try_move_pod(
        &self,
        kind: usize,
        id: usize,
        dest_x: usize,
        to_room: bool,
        rooms: &[[(usize, usize); 7]; 12],
        hallway: &[(usize, usize); 12],
        species_count: usize,
    ) -> Option<GameState> {
        let (x, y) = self.pods[kind][id];

        // If there's a different kind of pod in our destination room, we can't move in there yet
        if to_room {
            if rooms[POD_DEST[kind]][2..=2 + species_count]
                .iter()
                .find(|(other_kind, _)| *other_kind != kind && *other_kind != usize::MAX)
                .is_some()
            {
                return None;
            }
        } else {
            // We're in a room, check if we can move out
            let pod_above = rooms[x as usize][2..y]
                .iter()
                .find(|(k, _)| *k != usize::MAX)
                .is_some();

            if pod_above {
                return None;
            }
        }

        let dest_y;

        if to_room {
            dest_y = (2..2 + species_count)
                .rev()
                .find(|y| rooms[POD_DEST[kind]][*y].0 == usize::MAX)
                .unwrap();
        } else {
            dest_y = 1;
        }

        // Check that the hallway path is not obstructed
        let walk_dir: i32 = if x < dest_x { 1 } else { -1 };
        let mut walk = x as i32;
        while walk != dest_x as i32 {
            // dbg!(*x, dest_x, walk);
            walk += walk_dir;
            if hallway[walk as usize].0 != usize::MAX {
                return None;
            }
        }

        Some(self.move_pod(kind, id, (dest_x, dest_y)))
    }

    fn is_done(&self, species_count: usize) -> bool {
        for (kind, pods) in self.pods.iter().enumerate() {
            for (id, (x, y)) in pods[..species_count].iter().enumerate() {
                if *x != POD_DEST[kind] {
                    return false;
                }
            }
        }
        true
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
    let part_one = move_pods(&starting_state, 2);

    lines.insert(3, String::from("  #D#C#B#A#"));
    lines.insert(4, String::from("  #D#B#A#C#"));

    let extended_state = parse_game_state(&lines);
    let part_two = move_pods(&extended_state, 4);

    (part_one, part_two)
}

fn move_pods(start: &GameState, species_count: usize) -> u32 {
    // Dijkstra
    let mut queue: BinaryHeap<GameState> = BinaryHeap::new();
    let mut visited: AHashMap<[[(usize, usize); 4]; 4], u32> = AHashMap::new();

    queue.push(start.clone());

    while let Some(next_state) = queue.pop() {
        if next_state.is_done(species_count) {
            return next_state.cost;
        }

        let (hallway, rooms) = next_state.generate_locations(species_count);

        'podloop: for (kind, pods) in next_state.pods.iter().enumerate() {
            for (id, (x, y)) in pods[..species_count].iter().enumerate() {
                // Handle some unreachable states
                if *x == POD_DEST[kind] {
                    // At the bottom of the destination room = don't move
                    if *y == 1 + species_count {
                        continue;
                    }
                    // At the top of the destination area, and the same pod type is in
                    // the same room below it (both are in the right spot)
                    if rooms[*x][*y + 1].0 == kind {
                        continue;
                    }
                }

                // We're in the hallway
                if *y == 1 {
                    if let Some(new_state) = next_state.try_move_pod(
                        kind,
                        id,
                        POD_DEST[kind],
                        true,
                        &rooms,
                        &hallway,
                        species_count,
                    ) {
                        if *visited.get(&new_state.pods).unwrap_or(&u32::MAX) > new_state.cost {
                            visited.insert(new_state.pods, new_state.cost);
                            queue.push(new_state);
                            break 'podloop;
                        }
                    }
                } else {
                    // We're in a room and need to move to the hallway
                    // Can't move to doorways (3, 5, 7, 9)
                    // dbg!(&next_state);
                    for dest_x in [1, 2, 4, 6, 8, 10, 11] {
                        if let Some(new_state) = next_state.try_move_pod(
                            kind,
                            id,
                            dest_x,
                            false,
                            &rooms,
                            &hallway,
                            species_count,
                        ) {
                            if *visited.get(&new_state.pods).unwrap_or(&u32::MAX) > new_state.cost {
                                visited.insert(new_state.pods, new_state.cost);
                                queue.push(new_state);
                            }
                        }
                    }
                }
            }
        }
    }

    unreachable!("We should have found a finished state!");
}

fn parse_game_state(lines: &Vec<String>) -> GameState {
    let mut parsed_pods: [[(usize, usize); 4]; 4] = [[(0xF, 0xF); 4]; 4];

    let mut counts = [0, 0, 0, 0];

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
                    "A" => (0, pos),
                    "B" => (1, pos),
                    "C" => (2, pos),
                    "D" => (3, pos),
                    _ => panic!("Unexpected pod type"),
                };

                parsed_pods[type_][counts[type_]] = parsed;
                counts[type_] += 1;
            }
        }
    }

    let mut state = GameState {
        pods: parsed_pods,
        cost: 0,
    };

    state
}
