use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::AddAssign,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec3D {
    x: i32,
    y: i32,
    z: i32,
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vec3D {
    fn len(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

struct Particle {
    id: usize,
    pos: Vec3D,
    vel: Vec3D,
    accel: Vec3D,
    from_center: usize,
    is_moving_away: bool,
}

impl Particle {
    fn step(&mut self) {
        let previous_from_center = self.from_center;

        self.vel += self.accel;
        self.pos += self.vel;
        self.from_center = self.pos.len();

        // We're moving away if we were previously closer to the center
        self.is_moving_away = previous_from_center < self.from_center;
    }

    /**
    * Rough approximation of collision prediction. Only works because the edge case this algorithm
    * *doesn't* check must be true in all 3 axes. Basically:
    *   In a given axis, if A is ahead of B in either direction, and B cannot catch up
    *   to A according to their velocities or their accelerations, then A can never collide with B.
    *
    * In reality, they still might collide if the relative acceleration of B does not dampen the relative
    * velocity quickly enough. 
    */
    fn can_collide(&self, other: &Self) -> bool {
        let x_rel = other.vel.x - self.vel.x;
        let y_rel = other.vel.x - self.vel.x;
        let z_rel = other.vel.x - self.vel.x;

        if !(x_rel < 0 && self.pos.x < other.pos.x && self.accel.x < other.accel.x)
            && !(x_rel > 0 && self.pos.x > other.pos.x && self.accel.x > other.accel.x)
        {
            return false;
        }
        if !(y_rel < 0 && self.pos.y < other.pos.y && self.accel.y < other.accel.y)
            && !(y_rel > 0 && self.pos.y > other.pos.y && self.accel.y > other.accel.y)
        {
            return false;
        }
        if !(z_rel < 0 && self.pos.z < other.pos.z && self.accel.z < other.accel.z)
            && !(z_rel > 0 && self.pos.z > other.pos.z && self.accel.z > other.accel.z)
        {
            return false;
        }

        return true;
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut particles: Vec<Particle> = read_particles(false);

    while !particles.iter().all(|p| p.is_moving_away) {
        for p in particles.iter_mut() {
            p.step();
        }
    }

    // Once all particles are moving away from the center, the one which will eventually be closest
    // is the one with the lowest acceleration as all other particles will eventually move faster than it
    let closest_long_term = particles
        .iter()
        .min_by(|a, b| a.accel.len().cmp(&b.accel.len()))
        .unwrap();

    println!(
        "Particle {} is closest to the center in the long term",
        closest_long_term.id
    )
}

fn part_two() {
    let mut particles: Vec<Particle> = read_particles(false);
    let mut collisions_finished = false;

    while !collisions_finished {
        collisions_finished = true;
        for i in 0..particles.len() - 1 {
            for j in i..particles.len() {
                if particles[i].can_collide(&particles[j]) {
                    collisions_finished = false;
                    break;
                }

                if !collisions_finished {
                    break;
                }
            }
        }

        for p in particles.iter_mut() {
            p.step();
        }

        let collisions = get_collisions(&particles);
        particles = particles
            .into_iter()
            .filter(|p| !collisions.contains(&p.id))
            .collect();
    }

    println!("{} particles are left after collisions", particles.len())
}

fn get_collisions(particles: &[Particle]) -> HashSet<usize> {
    let mut occupied: HashMap<Vec3D, Vec<usize>> = HashMap::new();

    for p in particles {
        let occupiers = occupied.entry(p.pos).or_insert(Vec::new());
        occupiers.push(p.id);
    }

    let mut collisions = HashSet::new();

    for multi_occupied in occupied.values().filter(|v| v.len() > 1) {
        for id in multi_occupied {
            collisions.insert(*id);
        }
    }

    return collisions;
}

fn parse_particle(enumeration: (usize, &str)) -> Particle {
    let (id, line) = enumeration;

    let mut parts = line.split(", ");

    let p = parts.next().unwrap();
    let v = parts.next().unwrap();
    let a = parts.next().unwrap();

    let mut p = p[3..p.len() - 1].split(",");
    let mut v = v[3..v.len() - 1].split(",");
    let mut a = a[3..a.len() - 1].split(",");

    let pos = Vec3D {
        x: p.next().unwrap().parse().unwrap(),
        y: p.next().unwrap().parse().unwrap(),
        z: p.next().unwrap().parse().unwrap(),
    };
    let vel = Vec3D {
        x: v.next().unwrap().parse().unwrap(),
        y: v.next().unwrap().parse().unwrap(),
        z: v.next().unwrap().parse().unwrap(),
    };
    let accel = Vec3D {
        x: a.next().unwrap().parse().unwrap(),
        y: a.next().unwrap().parse().unwrap(),
        z: a.next().unwrap().parse().unwrap(),
    };

    Particle {
        id,
        pos,
        vel,
        accel,
        from_center: pos.len(),
        is_moving_away: false,
    }
}

fn read_particles(test: bool) -> Vec<Particle> {
    let path = if test { "./src/test" } else { "./src/input" };
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(parse_particle)
        .collect()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_particle() {
        let input = "p=<-1724,-1700,5620>, v=<44,-10,-107>, a=<2,6,-9>";
        let particle = parse_particle((0, input));

        // Test ID
        assert_eq!(particle.id, 0);

        // Test Position
        assert_eq!(
            particle.pos,
            Vec3D {
                x: -1724,
                y: -1700,
                z: 5620
            }
        );

        // Test Velocity
        assert_eq!(
            particle.vel,
            Vec3D {
                x: 44,
                y: -10,
                z: -107
            }
        );

        // Test Acceleration
        assert_eq!(particle.accel, Vec3D { x: 2, y: 6, z: -9 });

        // Test Distance from Center
        assert_eq!(particle.from_center, 1724 + 1700 + 5620);
    }

    #[test]
    fn test_particle_step() {
        let mut particle = parse_particle((0, "p=<3,0,0>, v=<2,0,0>, a=<-1,1,0>"));

        particle.step();

        // Test Position
        assert_eq!(particle.pos, Vec3D { x: 4, y: 1, z: 0 });

        // Test Velocity
        assert_eq!(particle.vel, Vec3D { x: 1, y: 1, z: 0 });

        // Test Distance from Center
        assert_eq!(particle.from_center, 5);
    }

    #[test]
    fn test_get_collisions() {
        let mut particles = Vec::new();

        particles.push(parse_particle((0, "p=<3,0,0>, v=<2,0,0>, a=<-1,1,0>")));
        particles.push(parse_particle((1, "p=<3,0,0>, v=<2,0,0>, a=<-1,1,0>")));

        let collisions = get_collisions(&particles);

        assert_eq!(collisions.len(), 2);
    }

    #[test]
    fn test_particle_can_collide() {
        let p0 = parse_particle((0, "p=<6,0,0>, v=<2,0,0>, a=<-0,0,0>"));
        let p1 = parse_particle((1, "p=<4,0,0>, v=<2,0,0>, a=<-1,0,0>"));
        let p2 = parse_particle((2, "p=<3,0,0>, v=<4,0,0>, a=<-3,0,0>"));

        assert_eq!(p0.can_collide(&p1), false);
        assert_eq!(p0.can_collide(&p2), true);
    }
}
