#![allow(dead_code, unused)]

use std::{
    char,
    collections::{HashMap, HashSet},
    fmt::Pointer,
};

fn main() {
    let inp = include_str!("input1.txt");
    let out = process(inp);
    dbg!(out);
}

fn process(inp: &str) -> i32 {
    let mut total_ans: i32 = 0;
    for line in inp.lines() {
        let (_, rest_of_the_line) = line.split_once(": ").unwrap();
        let (winning_nums_str, our_nums_str) = rest_of_the_line.split_once(" | ").unwrap();
        let winning_nums = winning_nums_str
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let our_nums = our_nums_str
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let winning_set: HashSet<usize> = HashSet::from_iter(winning_nums);
        let mut matching_nums = 0;
        for num in our_nums {
            if winning_set.contains(&num) {
                matching_nums += 1;
            }
        }
        let base: i32 = 2;
        if matching_nums > 0 {
            total_ans += base.pow(matching_nums - 1);
        }
    }
    total_ans
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_process() {
        let inp = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(inp);
        assert_eq!(result, 13);
    }
}
