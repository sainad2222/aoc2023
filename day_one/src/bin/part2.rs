#![allow(dead_code, unused)]

use std::collections::HashMap;
fn main() {
    let inp = include_str!("input2.txt");
    let out = bruteforce(inp);
    dbg!(out);
}

fn compute_lps(needle: &str) -> Vec<usize> {
    let mut lps: Vec<usize> = vec![0; needle.len()];
    let mut length = 0;
    let mut i = 1;
    while i < needle.len() {
        if needle.as_bytes()[i] == needle.as_bytes()[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            // if didn't match and we have a possibility of having smaller prefix
            // reset length to next longest possible and don't increment i
            if length > 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}

fn find_multiple_occurences_using_kmp(needle: &str, haystack: &str) -> Vec<usize> {
    let n = haystack.len();
    let m = needle.len();
    let lps = compute_lps(needle);
    let mut occurences: Vec<usize> = Default::default();
    let mut i = 0;
    let mut j = 0;
    while i < n {
        if haystack.as_bytes()[i] == needle.as_bytes()[j] {
            i += 1;
            j += 1;
        }

        if j == m {
            occurences.push(i - j);
            // reset
            // didn't fully understand this part tho
            j = lps[j - 1];
        } else if i < n && needle.as_bytes()[j] != haystack.as_bytes()[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    occurences
}

#[derive(Default, Debug)]
struct CacheEntry<'a> {
    first_occurence: OccurenceEntry<'a>,
    last_occurence: OccurenceEntry<'a>,
}

#[derive(Default, Debug)]
struct OccurenceEntry<'a> {
    occurence_idx: usize,
    occurene_word: &'a str,
}

fn bruteforce(inp: &str) -> u32 {
    let valid_digit_strings = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let word_to_digit_mapping: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut total_sum: u32 = 0;
    for line in inp.lines() {
        let mut digit_occurences: HashMap<&str, CacheEntry> = Default::default();
        for valid_digit_string in valid_digit_strings.clone() {
            let cur_digit_occurences = find_multiple_occurences_using_kmp(valid_digit_string, line);
            if !cur_digit_occurences.is_empty() {
                digit_occurences.insert(
                    valid_digit_string,
                    CacheEntry {
                        first_occurence: OccurenceEntry {
                            occurence_idx: cur_digit_occurences[0],
                            occurene_word: valid_digit_string,
                        },
                        last_occurence: OccurenceEntry {
                            occurence_idx: cur_digit_occurences[cur_digit_occurences.len() - 1],
                            occurene_word: valid_digit_string,
                        },
                    },
                );
            }
        }
        let mut first_indices: Vec<_> = digit_occurences.values().collect();
        first_indices.sort_by(|a, b| {
            a.first_occurence
                .occurence_idx
                .cmp(&b.first_occurence.occurence_idx)
        });
        let mut last_indices: Vec<_> = digit_occurences.values().collect();
        last_indices.sort_by(|a, b| {
            b.last_occurence
                .occurence_idx
                .cmp(&a.last_occurence.occurence_idx)
        });
        let first_word = first_indices[0].first_occurence.occurene_word;
        let last_word = last_indices[0].last_occurence.occurene_word;
        total_sum += (word_to_digit_mapping[first_word] * 10 + word_to_digit_mapping[last_word]);
    }
    total_sum
}

fn optimal(inp: &str) -> u32 {
    let _valid_digit_strings = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    0
}

#[cfg(test)]
mod tests {
    use crate::{bruteforce, optimal};

    #[test]
    fn test_process() {
        let inp = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = bruteforce(inp);
        assert_eq!(result, 281);
    }
}
