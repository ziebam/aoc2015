fn part1(input: &str) -> i32 {
    let mut out = 0;
    for c in input.chars() {
        if c == '(' {
            out += 1;
        } else {
            out -= 1;
        }
    }

    out
}

fn part2(input: &str) -> i32 {
    let mut pos = 0;
    let mut floor = 0;
    for c in input.chars() {
        pos += 1;
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 {
            return pos;
        }
    }

    eprintln!("Didn't reach the basement.");
    std::process::exit(1);
}

pub fn solve() {
    let input = include_str!("../input/in1.txt");

    let start = std::time::Instant::now();
    let p1 = part1(&input);
    let p1_time = start.elapsed();
    let p2 = part2(&input);
    let p2_time = start.elapsed() - p1_time;

    println!(
        "Part 1 answer: \x1b[92m{}\x1b[0m. Took ~{} microseconds.",
        p1,
        p1_time.as_nanos() as f32 / 1000.0
    );
    println!(
        "Part 1 answer: \x1b[92m{}\x1b[0m. Took ~{} microseconds.",
        p2,
        p2_time.as_nanos() as f32 / 1000.0
    );
}
