use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
};

#[derive(Debug)]
struct Relation {
    source: String,
    target: String,
}

type Dependencies = HashMap<String, HashSet<String>>;

struct Worker {
    task: Option<String>,
    remaining: i32,
}

impl Worker {
    fn is_done(&self) -> bool {
        self.task.is_none() || (!self.task.is_none() && self.remaining == 0)
    }

    fn start(&mut self, task: String, length: i32) {
        self.task = Some(task);
        self.remaining = length;
    }
}

fn main() {
    let now = Instant::now();
    let relations: Vec<Relation> = fs::read_to_string("./src/day07.txt")
        .unwrap()
        .lines()
        .map(parse_relation)
        .collect();

    part_one(&relations);
    part_two(&relations);
    println!("{}ms", now.elapsed().as_millis());
}

fn part_one(relations: &Vec<Relation>) {
    let mut dependency_map = build_dependency_map(relations);
    let mut order: VecDeque<String> = VecDeque::new();

    loop {
        // dbg!(&dependency_map);
        match get_next(&dependency_map, &order) {
            Some(s) => {
                order.push_back(String::from(&s));
                dependency_map.remove(&s);
            }
            None => break,
        }
    }
    println!("{}", order.make_contiguous().join(""))
}

fn part_two(relations: &Vec<Relation>) {
    let mut dependency_map = build_dependency_map(relations);
    let mut order: VecDeque<String> = VecDeque::new();
    let mut workers: Vec<Worker> = (0..5)
        .map(|_| Worker {
            task: None,
            remaining: -1,
        })
        .collect();

    // Assign initial tasks
    for worker in workers.iter_mut() {
        match get_next(&dependency_map, &order) {
            Some(s) => {
                dependency_map.remove(&s);
                assign_task(worker, s);
            }
            None => continue,
        }
    }

    let mut tick = 0;

    while dependency_map.keys().len() != 0 || workers.iter().any(|w| !w.is_done()) {
        for worker in workers.iter_mut() {
            if worker.is_done() {
                match &worker.task {
                    Some(t) => order.push_back(String::from(t)),
                    None => {}
                }

                match get_next(&dependency_map, &order) {
                    Some(s) => {
                        dependency_map.remove(&s);
                        assign_task(worker, s);
                    }
                    None => {}
                }
            } else {
                worker.remaining -= 1;
            }
        }
        tick += 1;
    }

    // OBO for the example lol
    println!("Time to complete: {}", tick) // + 1 for the sample
}

fn assign_task(worker: &mut Worker, task: String) {
    let time = (task.chars().next().unwrap() as u32) - 64 + 60;
    // println!("Assigning {} for {} seconds", task, time);
    worker.start(String::from(task), time as i32 - 1);
}

fn get_next(dependencies: &Dependencies, have: &VecDeque<String>) -> Option<String> {
    let mut choices: Vec<&String> = dependencies
        .iter()
        .filter(|(_, depends)| all(&depends, have))
        .map(|(choice, _)| choice)
        .collect();

    choices.sort();

    match choices.first() {
        Some(s) => Some(String::from(*s)),
        None => None,
    }
}

fn all(items: &HashSet<String>, in_: &VecDeque<String>) -> bool {
    for item in items {
        if !in_.contains(item) {
            return false;
        }
    }

    return true;
}

fn build_dependency_map(relations: &Vec<Relation>) -> Dependencies {
    let mut dependency_map: Dependencies = HashMap::new();

    // Add nodes that have dependencies
    for rel in relations {
        let set = dependency_map
            .entry(String::from(&rel.target))
            .or_insert(HashSet::new());
        set.insert(String::from(&rel.source));
    }

    // Add nodes without dependencies
    for rel in relations {
        dependency_map
            .entry(String::from(&rel.source))
            .or_insert(HashSet::new());
    }

    dependency_map
}

fn parse_relation(line: &str) -> Relation {
    let mut words = line.split(" ");
    let source = String::from(words.nth(1).unwrap());
    let target = String::from(words.nth(5).unwrap());

    Relation { source, target }
}
