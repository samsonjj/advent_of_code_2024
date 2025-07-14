use advent_of_code_2024::aoc;

fn main() {
    let input = include_str!("example.txt");
    println!("part_1: {}", aoc::format_with_time(|| part_1(input)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(input)));
}

fn print_blocks(blocks: &Vec<Option<i64>>) {
    for &item in blocks.iter() {
        print!("{}", item.map(|x| (x % 10) as u8 + '0' as u8).unwrap_or('.' as u8) as char);
    }
    println!();
}

fn checksum(blocks: &Vec<Option<i64>>) -> i64 {
    let mut sum = 0;
    for (i, item) in blocks.iter().enumerate() {
        if let Some(file_id) = item {
            sum += *file_id * i as i64;
        }
    }
    sum
}

fn part_1(input: &str) -> i64 {
    let mut blocks: Vec<Option<i64>> = vec![];
    let mut file_id = 0;
    let mut is_file = true; // false if free space
    for c in input.chars() {
        // append correct number of file ids
        let num = c as i64 - '0' as i64;
        for i in 0..num {
            blocks.push(if is_file { Some(file_id)} else { None });
        }

        // adjust flags
        is_file = !is_file;
        if is_file {
            file_id += 1;
        }
    }

    let mut front = 0;
    let mut back = blocks.len()-1;

    print_blocks(&blocks);

    loop {
        if front == back {
            break;
        }

        if blocks[front].is_some() {
            front += 1;
            continue;
        }

        if blocks[back].is_none() {
            back -= 1;
            continue;
        }

        (blocks[front], blocks[back]) = (blocks[back], None);
    }

    print_blocks(&blocks);

    checksum(&blocks)
}

fn part_2(input: &str) -> i64 {
    let mut blocks: Vec<Option<i64>> = vec![];
    let mut file_id = 0;
    let mut is_file = true; // false if free space
    for c in input.chars() {
        // append correct number of file ids
        let num = c as i64 - '0' as i64;
        for i in 0..num {
            blocks.push(if is_file { Some(file_id)} else { None });
        }

        // adjust flags
        is_file = !is_file;
        if is_file {
            file_id += 1;
        }
    }

    let mut front = 0;
    let mut back = blocks.len()-1;

    print_blocks(&blocks);

    loop {
        if front == back {
            break;
        }

        if blocks[front].is_some() {
            front += 1;
            continue;
        }

        if blocks[back].is_none() {
            back -= 1;
            continue;
        }

        (blocks[front], blocks[back]) = (blocks[back], None);
    }

    print_blocks(&blocks);

    checksum(&blocks)
}
