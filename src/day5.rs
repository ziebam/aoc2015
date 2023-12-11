fn is_nice_part1(string: &str) -> i32 {
    let mut chars = string.chars();

    let mut num_vowels = 0;
    let mut has_letter_twice_in_a_row = false;

    for (i, c) in string.chars().enumerate() {
        if "aeiou".contains(c) {
            num_vowels += 1;
        }

        if i == 0 {
            continue;
        }

        let prev_char = chars.nth(0).unwrap();
        match c {
            'b' => {
                if prev_char == 'a' {
                    return 0;
                }
            }
            'd' => {
                if prev_char == 'c' {
                    return 0;
                }
            }
            'q' => {
                if prev_char == 'p' {
                    return 0;
                }
            }
            'y' => {
                if prev_char == 'x' {
                    return 0;
                }
            }
            _ => (),
        }

        if !has_letter_twice_in_a_row && c == prev_char {
            has_letter_twice_in_a_row = true;
        }
    }

    if num_vowels >= 3 && has_letter_twice_in_a_row {
        return 1;
    }

    0
}

fn part1(input: &str) -> i32 {
    let mut out = 0;
    for string in input.lines() {
        out += is_nice_part1(string);
    }

    out
}

fn is_nice_part2(string: &str) -> i32 {
    let chars: Vec<char> = string.chars().collect();

    let mut pairs: Vec<Vec<char>> = Vec::new();
    let mut has_two_pairs = false;
    let mut has_three_letter_palindrome = false;

    for i in 1..string.len() {
        for (j, pair) in pairs.iter().enumerate() {
            if pair[0] == chars[i - 1] && pair[1] == chars[i] && (i - j) > 2 {
                has_two_pairs = true;
                break;
            }
        }

        pairs.push(vec![chars[i - 1], chars[i]]);
    }

    for i in 2..string.len() {
        if chars[i - 2] == chars[i] {
            has_three_letter_palindrome = true;
            break;
        }
    }

    if has_three_letter_palindrome && has_two_pairs {
        return 1;
    }

    0
}

fn part2(input: &str) -> i32 {
    let mut out = 0;
    for string in input.lines() {
        out += is_nice_part2(string);
    }

    out
}

pub fn solve() {
    let input = include_str!("../input/in5.txt");

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
        "Part 2 answer: \x1b[92m{}\x1b[0m. Took ~{} milliseconds.\n",
        p2,
        p2_time.as_micros() as f32 / 1000.0
    );
}
