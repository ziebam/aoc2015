fn part1(input: &str) -> i32 {
    let mut out = 0;
    for present in input.lines() {
        let mut lwh = present.split('x');
        let (l, w, h) = (
            lwh.next().unwrap().parse::<i32>().unwrap(),
            lwh.next().unwrap().parse::<i32>().unwrap(),
            lwh.next().unwrap().parse::<i32>().unwrap(),
        );

        let lw = l * w;
        let wh = w * h;
        let lh = l * h;
        let min = *vec![lw, wh, lh].iter().min().unwrap();

        out += 2 * lw + 2 * wh + 2 * lh + min;
    }

    out
}

fn part2(input: &str) -> i32 {
    let mut out = 0;
    for present in input.lines() {
        let mut lwh = present.split('x');
        let (l, w, h) = (
            lwh.next().unwrap().parse::<i32>().unwrap(),
            lwh.next().unwrap().parse::<i32>().unwrap(),
            lwh.next().unwrap().parse::<i32>().unwrap(),
        );

        let mut lwh_vec = vec![l, w, h];
        lwh_vec.sort();

        let (min1, min2) = (lwh_vec[0], lwh_vec[1]);

        out += 2 * min1 + 2 * min2 + l * w * h
    }

    out
}

pub fn solve() {
    let input = include_str!("../input/in2.txt");

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
        "Part 2 answer: \x1b[92m{}\x1b[0m. Took ~{} microseconds.\n",
        p2,
        p2_time.as_nanos() as f32 / 1000.0
    );
}
