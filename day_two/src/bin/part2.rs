#![allow(dead_code, unused)]

use std::char;

fn main() {
    let inp = include_str!("input1.txt");
    let out = process(inp);
    dbg!(out);
}

// return: rgb
fn process_each_line(line: &str) -> (usize, usize, usize) {
    let (_, rest_of_the_line) = line.split_once(": ").unwrap();
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for turn in rest_of_the_line.split("; ") {
        for num_and_color in turn.split(", ") {
            let (num, color) = num_and_color.split_once(' ').unwrap();
            let num_as_int = num.parse::<usize>().unwrap();
            match color {
                "red" => {
                    red = red.max(num_as_int);
                }
                "green" => {
                    green = green.max(num_as_int);
                }
                "blue" => {
                    blue = blue.max(num_as_int);
                }
                _ => return (0, 0, 0),
            }
        }
    }
    (red, green, blue)
}

fn process(inp: &str) -> usize {
    let mut total_ans: usize = 0;
    for (_, line) in inp.lines().enumerate() {
        let (red, green, blue) = process_each_line(line);
        total_ans += (red * green * blue);
    }
    total_ans
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_process() {
        let inp = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(inp);
        assert_eq!(result, 2286);
    }
}
