use std::collections::{HashMap, HashSet};

use crate::aocutil::Grid;

pub fn solve(forest: Vec<Vec<u8>>) -> (usize, usize) {
    (
        get_visible(&forest).len(),
        *get_scores(&forest).values().max().unwrap(),
    )
}
fn get_scores(forest: &Vec<Vec<u8>>) -> HashMap<(usize, usize), usize> {
    let mut tree_scores: HashMap<(usize, usize), usize> = HashMap::new();
    let width = forest.width();
    let height = forest.height();

    for (y, row) in forest.iter().enumerate().take(height) {
        // Scan L->R
        get_visibility_score(&mut row[0..width].iter())
            .iter()
            .enumerate()
            .for_each(|(x, score)| {
                *tree_scores.entry((x, y)).or_insert(1) *= *score;
            });
        // Scan R->L
        get_visibility_score(&mut row[0..width].iter().rev())
            .iter()
            .enumerate()
            .for_each(|(x, score)| {
                *tree_scores.entry((width - 1 - x, y)).or_insert(1) *= *score;
            });
    }

    // Transpose to make L->R become B->T
    let rotated = forest.rot90();
    for (x, column) in rotated.iter().enumerate().take(width) {
        // Scan B->T
        get_visibility_score(&mut column[0..height].iter())
            .iter()
            .enumerate()
            .for_each(|(y, score)| {
                *tree_scores.entry((x, height - 1 - y)).or_insert(1) *= *score;
            });
        // Scan T->B
        get_visibility_score(&mut column[0..height].iter().rev())
            .iter()
            .enumerate()
            .for_each(|(y, score)| {
                *tree_scores.entry((x, y)).or_insert(1) *= *score;
            });
    }
    tree_scores
}

fn get_visibility_score<'a, I>(iter: &mut I) -> Vec<usize>
where
    I: Iterator<Item = &'a u8>,
{
    // Keep track of the last time a tree of a given height was seen
    let mut tree_size_last_seen: HashMap<u8, usize> = HashMap::new();
    let mut visibility_scores: Vec<usize> = Vec::new();

    for (idx, current_tree) in iter.enumerate() {
        // Find the most recent tree with a current or greater height
        // otherwise we don't have one, and we can achieve a 'maximum' score for this position
        let most_recent_blocking_distance = idx
            - *tree_size_last_seen
                .iter()
                .filter(|(tree, _)| *tree >= current_tree)
                .map(|(_, last_idx)| last_idx)
                .max()
                .unwrap_or(&0);

        visibility_scores.push(most_recent_blocking_distance);
        tree_size_last_seen.insert(*current_tree, idx);
    }

    visibility_scores
}

fn get_visible(forest: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let width = forest.width();
    let height = forest.height();

    let mut visible = HashSet::new();
    for (y, row) in forest.iter().enumerate().take(height) {
        // Scan L->R
        get_visible_in_iter(&mut row[0..width].iter())
            .iter()
            .for_each(|x| {
                visible.insert((*x, y));
            });
        // Scan R->L
        get_visible_in_iter(&mut row[0..width].iter().rev())
            .iter()
            .for_each(|x| {
                visible.insert((width - 1 - *x, y));
            });
    }

    // Transpose to make L->R become B->T
    let rotated = forest.rot90();
    for (x, column) in rotated.iter().enumerate().take(width) {
        // Scan B->T
        get_visible_in_iter(&mut column[0..height].iter())
            .iter()
            .for_each(|y| {
                visible.insert((x, height - 1 - *y));
            });
        // Scan T->B
        get_visible_in_iter(&mut column[0..height].iter().rev())
            .iter()
            .for_each(|y| {
                visible.insert((x, *y));
            });
    }

    visible
}

fn get_visible_in_iter<'a, I>(iter: &mut I) -> Vec<usize>
where
    I: Iterator<Item = &'a u8>,
{
    let mut visible = Vec::new();
    let mut largest: u8 = 0;
    for (pos, height) in iter.enumerate() {
        if *height > largest || pos == 0 {
            visible.push(pos);
            largest = *height;
        }
    }
    visible
}
