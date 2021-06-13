use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();
    let original = fs::read_to_string("./src/input").unwrap();

    let stream = String::from(&original);

    println!("Part 1: {}", collapse(&stream));

    let alphabet: Vec<char> = (b'a'..=b'z').map(|c| c as char).collect();

    let mut lengths: Vec<usize> = Vec::new();

    for char in alphabet {
        let stream: String = String::from(&original)
            .chars()
            .filter(|c| c != &char && c != &char.to_ascii_uppercase())
            .collect();

        &lengths.push(collapse(&stream));
    }

    println!("Part 2: {:?}", lengths.iter().min().unwrap());

    println!("{}ms", now.elapsed().as_millis());
}

fn collapse(seq: &String) -> usize {
    let mut new_string: Vec<char> = Vec::new();

    let mut stream = seq.chars();

    new_string.push(stream.next().unwrap());

    for c in stream {
        if new_string.len() > 0 && (*new_string.last().unwrap() as i32 - c as i32).abs() == 32 {
            new_string.pop();
        } else {
            &new_string.push(c);
        };
    }

    new_string.len()
}
