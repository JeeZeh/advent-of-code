#[derive(Clone, Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn parse(s: &str) -> Self {
        let points: Vec<&str> = s.split(" -> ").collect();
        let a: Vec<&str> = points[0].split(",").collect();
        let b: Vec<&str> = points[1].split(",").collect();

        Line {
            x1: a[0].parse().unwrap(),
            y1: a[1].parse().unwrap(),
            x2: b[0].parse().unwrap(),
            y2: b[1].parse().unwrap(),
        }
    }

    fn get_points_iter(&self) -> Box<dyn Iterator<Item = (i32, i32)> + '_> {
        let (x1, y1, x2, y2) = (self.x1, self.y1, self.x2, self.y2);

        if x1 == x2 {
            return Box::new((y1.min(y2)..=y1.max(y2)).map(move |y| (x1, y)));
        } else if y1 == y2 {
            return Box::new((x1.min(x2)..=x1.max(x2)).map(move |x| (x, y1)));
        } else {
            return Box::new((0..=(x1 - x2).abs()).map(move |d| {
                (
                    if x1 > x2 { x1 - d } else { x1 + d },
                    if y1 > y2 { y1 - d } else { y1 + d },
                )
            }));
        }
    }
}

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let vents: Vec<Line> = lines.iter().map(|l| Line::parse(l)).collect();

    let (vert_or_horiz, diag) = vents.iter().partition(|l| l.x1 == l.x2 || l.y1 == l.y2);

    let mut overlaps: Vec<Vec<u8>> = vec![vec![0; 1000]; 1000];

    add_overlaps(&vert_or_horiz, &mut overlaps);
    let part_one = overlaps.iter().flatten().filter(|v| **v > 1).count();

    add_overlaps(&diag, &mut overlaps);
    let part_two = overlaps.iter().flatten().filter(|v| **v > 1).count();

    (part_one, part_two)
}

fn add_overlaps(vents: &Vec<&Line>, overlaps: &mut Vec<Vec<u8>>) {
    for vent in vents {
        for (x, y) in vent.get_points_iter() {
            overlaps[y as usize][x as usize] += 1;
        }
    }
}
