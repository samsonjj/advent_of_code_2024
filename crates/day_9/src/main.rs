mod mem;

use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
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
    let mut data = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i64).collect_vec();
    
    println!("start");

    // init record of all file locations
    let mut file_locations: HashMap<i64, i64> = HashMap::new();
    let mut file_lengths: HashMap<i64, i64> = HashMap::new();
    // i -> block_index
    let mut free_space_starts: HashMap<i64, i64> = HashMap::new();

    let mut block_index = 0i64;
    for i in 0..data.len() {
        if i % 2 == 0 {
        // store record:
            file_locations.insert(i as i64 / 2, block_index);
            file_lengths.insert(i as i64 / 2, data[i]);
        } else {
            free_space_starts.insert(i as i64, block_index);
        }
        block_index += data[i as usize];
    }
    
    println!("{file_locations:?}");

    // println!("{file_locations:?}");

    // file_id: keep track of the current file which we are trying to swap.
    // we will continuously decrement it by 1.
    let mut file_id = data.len() as i64 / 2;
    while file_id >= 0 {
        // get length of file
        let file_len = file_lengths.get(&file_id).unwrap();

        // iterate through free space
        for i in 0..data.len() {
            // continue if not free space
            if i % 2 == 0 {
                continue;
            }

            // continue if free space is not large enough
            let free_space_len = data[i];
            if free_space_len < *file_len {
                continue; 
            }

            // perform swap
            // free space length
            data[i] -= file_len;
            // file location
            file_locations.insert(file_id, *free_space_starts.get(&(i as i64)).unwrap());
            // free space start
            *free_space_starts.entry(i as i64).or_insert(0) += file_len;
            break;
        }
        file_id -= 1;
    }

    println!("{file_locations:?}");

    // compute checksum
    let mut sum = 0;
    for (file_id, file_location) in file_locations.iter() {
        let file_len = file_lengths.get(&file_id).unwrap();
        let value = (file_location + (file_location + file_len - 1)) * file_len / 2;
        dbg!(file_id, file_location, file_len, value * file_id);
        sum += value * file_id;
    }
    sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = 137;
        // 0..111....22222
        let result = part_2("12345");
    }
}
