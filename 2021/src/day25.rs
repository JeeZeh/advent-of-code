use crate::aocutil::Grid;

const DOWN: u8 = 118;
const RIGHT: u8 = 62;
const EMPTY: u8 = 46;

pub fn solve(mut grid: Vec<Vec<u8>>) -> (usize, String) {
    // let mut map = Map::from_grid(&input);

    let mut updated = true;
    let mut steps = 0;
    while updated {
        (updated, grid) = step(&grid);
        // grid.show_map(|f| match *f {
        //     RIGHT => '>',
        //     DOWN => 'v',
        //     EMPTY => '.',
        //     _ => panic!(),
        // });
        // dbg!();
        steps += 1;
    }

    (steps, String::from("Merry Christmas!"))
}

fn step(grid: &Vec<Vec<u8>>) -> (bool, Vec<Vec<u8>>) {
    let mut moved_right: Vec<Vec<u8>> = grid.clone();
    let (width, height) = (grid.width(), grid.height());

    let mut updated = false;

    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == RIGHT {
                let (dx, dy) = ((x + 1) % width, y);

                if grid[dy][dx] == EMPTY {
                    updated = true;
                    moved_right[dy][dx] = *col;
                    moved_right[y][x] = EMPTY;
                }
            }
        }
    }

    let mut moved_down = moved_right.clone();

    for (y, row) in moved_right.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == DOWN {
                let (dx, dy) = (x, (y + 1) % height);

                if moved_right[dy][dx] == EMPTY {
                    updated = true;
                    moved_down[dy][dx] = *col;
                    moved_down[y][x] = EMPTY;
                }
            }
        }
    }

    (updated, moved_down)
}
