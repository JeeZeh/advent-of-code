use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::aocutil::Grid;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos(usize, usize);

impl Pos {
    fn sum(&self) -> usize {
        self.0 + self.1
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Pos,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.sum().cmp(&other.position.sum()))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
struct Edge {
    node: Pos,
    cost: usize,
}

fn shortest_path(grid: &Vec<Vec<Edge>>, start: Pos, goal: Pos) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = vec![vec![usize::MAX; grid.width()]; grid.height()];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.1][start.0] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (dx, dy) in neighbours(position.0, position.1) {
            if let Some(edge) = grid.getyx(dy as usize, dx as usize) {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.position.1][next.position.0] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.position.1][next.position.0] = next.cost;
                }
            }
        }
    }

    // Goal not reachable
    None
}

/// ------ https://doc.rust-lang.org/std/collections/binary_heap/index.html ------ ///

pub fn solve(grid: Vec<Vec<u8>>) -> (usize, u64) {
    let grid: Vec<Vec<Edge>> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| Edge {
                    node: Pos(x, y),
                    cost: (*cell - 48) as usize,
                })
                .collect()
        })
        .collect();

    let part_one = shortest_path(&grid, Pos(0, 0), Pos(grid.width() - 1, grid.height() - 1));

    (part_one.unwrap(), 0)
}

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (i32, i32)> {
    [
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 + 1),
        (x as i32 - 1, y as i32),
        (x as i32, y as i32 - 1),
    ]
    .into_iter()
}
