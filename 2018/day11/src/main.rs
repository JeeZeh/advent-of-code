type Grid<T> = Vec<Vec<T>>;

struct FuelRack {
    serial_number: i32,
    fuel: Grid<Cell>,
    size: usize,
}

impl FuelRack {
    fn new(serial_number: i32, size: usize) -> FuelRack {
        let mut fuel: Grid<Cell> = Vec::with_capacity(size);

        for y in 1..=size {
            let mut row: Vec<Cell> = Vec::with_capacity(size);

            for x in 1..=size {
                row.push(Cell::new(x, y, serial_number));
            }

            fuel.push(row);
        }

        FuelRack {
            serial_number: serial_number,
            fuel,
            size,
        }
    }

    fn power_square(&self, start_x: usize, start_y: usize, square_size: usize) -> i32 {
        let mut power = 0;

        for d_y in 0..square_size {
            for d_x in 0..square_size {
                power += self.fuel[start_y + d_y][start_x + d_x].power;
            }
        }

        power
    }

    fn largest_power_square(&self, square_size: usize) -> (usize, usize, i32) {
        let mut largest: (usize, usize, i32) = (0, 0, 0);

        for y in 0..self.size - square_size {
            for x in 0..self.size - square_size {
                let power = self.power_square(x, y, square_size);

                if power > largest.2 {
                    largest = (x + 1, y + 1, power);
                }
            }
        }

        return largest;
    }
}

struct Cell {
    x_pos: i32,
    y_pos: i32,
    rack_id: i32,
    power: i32,
}

impl Cell {
    fn new(x: usize, y: usize, grid_serial_number: i32) -> Cell {
        let rack_id = x as i32 + 10;
        let power = (((rack_id * y as i32 + grid_serial_number) * rack_id) % 1000) / 100 - 5;

        Cell {
            x_pos: x as i32,
            y_pos: y as i32,
            rack_id,
            power,
        }
    }
}

fn main() {
    let rack = FuelRack::new(2568, 300);
    let (start_x, start_y, power) = rack.largest_power_square(3);

    println!(
        "Largest total power ({}) starts at {},{} (X,Y)",
        power, start_x, start_y
    );

    let mut largest: (usize, usize, i32, usize) = (0, 0, 0, 0);

    for size in 1..=300 {
        let (start_x, start_y, power) = rack.largest_power_square(size);

        if power > largest.2 {
            largest = (start_x, start_y, power, size)
        }
    }

    println!(
        "Largest total power ({}) has properties {},{},{} (X,Y,size)",
        largest.2, largest.0, largest.1, largest.3
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powers() {
        assert_eq!(Cell::new(122, 79, 57).power, -5);
        assert_eq!(Cell::new(217, 196, 39).power, 0);
        assert_eq!(Cell::new(101, 153, 71).power, 4);
    }

    #[test]
    fn test_largest_power_square() {
        assert_eq!(FuelRack::new(18, 300).largest_power_square(3), (33, 45, 29));
        assert_eq!(FuelRack::new(42, 300).largest_power_square(3), (21, 61, 30));
    }
}
