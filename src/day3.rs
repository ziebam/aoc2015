fn part1(input: &str) -> usize {
    let mut visited_houses = std::collections::HashSet::from([(0, 0)]);
    let mut x = 0;
    let mut y = 0;

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => eprintln!("Unexpected character."),
        }
        visited_houses.insert((x, y));
    }

    visited_houses.len()
}

fn part2(input: &str) -> usize {
    let mut visited_houses = std::collections::HashSet::from([(0, 0)]);
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => y1 += 1,
                'v' => y1 -= 1,
                '>' => x1 += 1,
                '<' => x1 -= 1,
                _ => eprintln!("Unexpected character."),
            }
            visited_houses.insert((x1, y1));
        } else {
            match c {
                '^' => y2 += 1,
                'v' => y2 -= 1,
                '>' => x2 += 1,
                '<' => x2 -= 1,
                _ => eprintln!("Unexpected character."),
            }
            visited_houses.insert((x2, y2));
        }
    }

    visited_houses.len()
}

pub fn solve() {
    let input = include_str!("../input/in3.txt");

    let start = std::time::Instant::now();
    let p1 = part1(&input);
    let p1_time = start.elapsed();
    let p2 = part2(&input);
    let p2_time = start.elapsed() - p1_time;

    println!(
        "Part 1 answer: \x1b[92m{}\x1b[0m. Took ~{} milliseconds.",
        p1,
        p1_time.as_micros() as f32 / 1000.0
    );
    println!(
        "Part 1 answer: \x1b[92m{}\x1b[0m. Took ~{} milliseconds.\n",
        p2,
        p2_time.as_micros() as f32 / 1000.0
    );
}
