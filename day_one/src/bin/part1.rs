use std::char;

fn main() {
    let inp = include_str!("input1.txt");
    let out = process(inp);
    dbg!(out);
}

fn process(inp: &str) -> u32 {
    let mut total_sum: u32 = 0;
    for line in inp.lines() {
        let mut numerics_chars: Vec<char> = Vec::default();
        for char in line.chars() {
            if char.is_numeric() {
                numerics_chars.push(char);
            }
        }
        let mut cur_line_number: String = Default::default();
        cur_line_number.push(numerics_chars[0]);
        cur_line_number.push(numerics_chars[numerics_chars.len() - 1]);
        let cur_line_number_as_digits: u32 = cur_line_number.parse().unwrap();
        total_sum += cur_line_number_as_digits;
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_process() {
        let inp = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(inp);
        assert_eq!(result, 142);
    }
}
