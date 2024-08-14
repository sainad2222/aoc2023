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

#[derive(Default, Debug)]
struct Number {
    i: usize,
    j: usize,
    val: usize,
}

fn crawl_left(matrix: Vec<Vec<char>>, i: usize, j: usize) -> Option<Number> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut cur_num: String = Default::default();
    let mut j = j;
    while j > 0 && matrix[i][j - 1].is_numeric() {
        cur_num.push(matrix[i][j - 1]);
        j -= 1;
    }
    let cur_num: String = cur_num.chars().rev().collect();
    if cur_num.is_empty() {
        return None;
    }
    let cur_num_as_num: usize = cur_num.parse().unwrap();
    Some(Number {
        i,
        j,
        val: cur_num_as_num,
    })
}

fn crawl_right(matrix: Vec<Vec<char>>, i: usize, j: usize) -> Option<Number> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut cur_num: String = Default::default();
    let mut j = j + 1;
    while j < m && matrix[i][j].is_numeric() {
        cur_num.push(matrix[i][j]);
        j += 1;
    }
    if cur_num.is_empty() {
        return None;
    }
    let cur_num_as_num: usize = cur_num.parse().unwrap();
    Some(Number {
        i,
        j,
        val: cur_num_as_num,
    })
}

// crawl_helper take (i,j) and returns number if there is a including number
// this is to help for top and bottom crawlers
fn crawl_helper(matrix: Vec<Vec<char>>, i: usize, j: usize) -> Option<Number> {
    if matrix[i][j] == '.' {
        return None;
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut left_string: String = Default::default();
    let mut right_string: String = Default::default();
    let mut new_j = j;
    while new_j < m && matrix[i][new_j].is_numeric() {
        right_string.push(matrix[i][new_j]);
        new_j += 1;
    }
    let mut j = j;
    while j > 0 && matrix[i][j - 1].is_numeric() {
        left_string.push(matrix[i][j - 1]);
        j -= 1;
    }
    let mut cur_num: String = left_string.chars().rev().collect();
    cur_num.push_str(&right_string);
    if cur_num.is_empty() {
        return None;
    }
    let cur_num_as_num: usize = cur_num.parse().unwrap();
    // i won't change and j is updated already
    Some(Number {
        i,
        j,
        val: cur_num_as_num,
    })
}

fn crawl_top_and_bottom(matrix: Vec<Vec<char>>, i: usize, j: usize) -> Vec<Number> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut final_vec: Vec<Number> = Vec::new();
    let mut positions: Vec<(usize, usize)> = Default::default();
    if i > 0 {
        positions.push((i - 1, j));
        if j > 0 {
            positions.push((i - 1, j - 1));
        }
        if j < m {
            positions.push((i - 1, j + 1));
        }
    }
    if i < n {
        positions.push((i + 1, j));
        if j > 0 {
            positions.push((i + 1, j - 1));
        }
        if j < m {
            positions.push((i + 1, j + 1));
        }
    }
    for &(x, y) in &positions {
        if let Some(number) = crawl_helper(matrix.clone(), x, y) {
            final_vec.push(number);
        }
    }
    final_vec
}

fn process(inp: &str) -> usize {
    let mut matrix: Vec<Vec<char>> = Default::default();
    for (idx, line) in inp.lines().enumerate() {
        matrix.push(line.chars().collect());
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut final_vec: Vec<Number> = Default::default();
    for i in 0..n {
        for j in 0..m {
            let cur_char = matrix[i][j];
            if cur_char == '.' {
                continue;
            }
            if !cur_char.is_numeric() {
                if let Some(num) = crawl_left(matrix.clone(), i, j) {
                    final_vec.push(num);
                }
                if let Some(num) = crawl_right(matrix.clone(), i, j) {
                    final_vec.push(num);
                }
                final_vec.extend(crawl_top_and_bottom(matrix.clone(), i, j).into_iter());
            }
        }
    }
    // dbg!(&final_vec);
    let mut max_value_map: HashMap<(usize, usize), usize> = Default::default();
    for num in final_vec {
        if let Some(cur_max) = max_value_map.get(&(num.i, num.j)) {
            if num.val > *cur_max {
                max_value_map.insert((num.i, num.j), num.val);
            }
        } else {
            max_value_map.insert((num.i, num.j), num.val);
        }
    }
    max_value_map.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_process() {
        let inp = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(inp);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_process_2() {
        let inp = "467*.114..
..........
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(inp);
        assert_eq!(result, 4326);
    }

    #[test]
    fn test_process_3() {
        let inp = "467..114..
...*...*..
..35..633.
......#...
617*......
.....+.58.
..592..*..
......755.
...$.*....
.664.598..";
        let result = process(inp);
        assert_eq!(result, 4533);
    }

    #[test]
    fn test_process_4() {
        let inp = "467..114..
..........
..35..633.
..........
617.......
......*58.
..592.....
......755.
..........
.664.598..";
        let result = process(inp);
        assert_eq!(result, 58);
    }
}
