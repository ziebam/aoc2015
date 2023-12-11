fn part1(input: &str) -> i32 {
    let mut i = 0;
    loop {
        let seed = [input, &i.to_string()].join("");
        let digest = md5::compute(seed);
        let stringified = format!("{:x}", digest);
        if stringified.starts_with("00000") {
            return i;
        }
        i += 1
    }
}

fn part2(input: &str) -> i32 {
    let mut i = 0;
    loop {
        let seed = [input, &i.to_string()].join("");
        let digest = md5::compute(seed);
        let stringified = format!("{:x}", digest);
        if stringified.starts_with("000000") {
            return i;
        }
        i += 1
    }
}

pub fn solve() {
    let input = include_str!("../input/in4.txt");

    let start = std::time::Instant::now();
    let p1 = part1(&input);
    let p1_time = start.elapsed();
    let p2 = part2(&input);
    let p2_time = start.elapsed() - p1_time;

    println!(
        "Part 1 answer: \x1b[92m{}\x1b[0m. Took ~{} seconds.",
        p1,
        p1_time.as_millis() as f32 / 1000.0
    );
    println!(
        "Part 2 answer: \x1b[92m{}\x1b[0m. Took ~{} seconds.\n",
        p2,
        p2_time.as_millis() as f32 / 1000.0
    );
}
