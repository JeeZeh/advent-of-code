use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();
    let original = fs::read_to_string("./src/input").unwrap();

    let mut stream = String::from(&original);
    while collapse(&mut stream) {}

    println!("Part 1: {}", stream.len());

    let alphabet = (b'a'..=b'z')
        .map(|c| {
            c as char
        })          
        .collect::<Vec<_>>();

    let mut lengths: Vec<usize> = Vec::new();

    for char in alphabet {
        let mut stream: String = String::from(&original).chars().filter(
            |c| c != &char && c != &char.to_ascii_uppercase()
        ).collect();
        
        while collapse(&mut stream) {}
        &lengths.push(stream.len());
    }

    println!("{:?}", lengths.iter().min().unwrap());

    println!("{}ms", now.elapsed().as_millis());
}

fn collapse(seq: &mut String) -> bool {
    let mut new_string = String::new();

    let mut stream = seq.chars().peekable();

    while let Some(chr) = stream.next() {
        let next = *stream.peek().unwrap_or(&'0');
        if (chr as i32 - next as i32).abs() == 32 {
            stream.next();
            continue;
        }
        new_string.push(chr);
    }

    let old_len = seq.len();

    seq.replace_range(0..seq.len(), &new_string[..]);
    old_len != seq.len()
}
