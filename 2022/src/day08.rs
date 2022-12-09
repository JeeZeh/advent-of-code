use std::collections::{HashSet, HashMap};

use crate::aocutil::Grid;

pub fn solve(forest: Vec<Vec<u8>>) -> (usize, usize) {
    let visible = get_visible(&forest);
    (get_visible(&forest).len(), 0)
}
fn get_visibility_scores<'a, I>(forest: &Vec<Vec<u8>>)
where
    I: Iterator<Item = &'a u8>,
{
    let mut tree_distances: HashMap<u8, u8> = HashMap::new();
    let visbility_scores: Vec<usize> = Vec::new();
    let width = forest.width();
    let height = forest.height();

    for y in 0..height {
        // get_visible_in_iter(&mut forest[y][0..width].iter())
        //     .iter()
        //     .for_each(|x| {
        //         visible.insert((*x, y));
        //     });
        // get_visible_in_iter(&mut forest[y][0..width].iter().rev())
        //     .iter()
        //     .for_each(|x| {
        //         visible.insert((width - 1 - *x, y));
        //     });
    }


}

fn get_visible(forest: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let width = forest.width();
    let height = forest.height();

    let mut visible = HashSet::new();
    for y in 0..height {
        get_visible_in_iter(&mut forest[y][0..width].iter())
            .iter()
            .for_each(|x| {
                visible.insert((*x, y));
            });
        get_visible_in_iter(&mut forest[y][0..width].iter().rev())
            .iter()
            .for_each(|x| {
                visible.insert((width - 1 - *x, y));
            });
    }

    let rotated = forest.rot90();
    for x in 0..width {
        get_visible_in_iter(&mut rotated[x][0..height].iter())
            .iter()
            .for_each(|y| {
                visible.insert((x, height - 1 - *y));
            });
        get_visible_in_iter(&mut rotated[x][0..height].iter().rev())
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
