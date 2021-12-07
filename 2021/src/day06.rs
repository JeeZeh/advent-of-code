pub fn solve(line: String) -> (u64, u64) {
    let mut fish: [u64; 9] = [0; 9];

    line.split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .for_each(|n| fish[n] += 1);

    (
        do_step(&mut fish, 80).iter().sum(),
        do_step(&mut fish, 256 - 80).iter().sum(),
    )
}

fn do_step(fish: &mut [u64; 9], n: usize) -> &mut [u64; 9] {
    // This works by holding the count of each fish at a given
    // internal counter in an array of 'bins' where fish[1] is the number
    // of fish with an internal counter of 1

    // Every day that passes, all fish have their counters decremented by
    // rotating the array to the left (fish at counter 1 become counter 0, 0 become 8, etc.).

    // The fish now at counter 8 represent both the new fish (they're left at 8) and their
    // parents which should actually reset to counter 6, so they're copied to index 6 of the array.
    for _ in 0..n {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish
}
