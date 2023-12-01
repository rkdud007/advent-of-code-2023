use std::{collections::HashMap, fs};

use regex::{Captures, Regex};

fn main() {
    let file_path = "input.txt";
    println!("Infile {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let re_front = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_back = Regex::new("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let result_front = re_front.replace_all(contents.as_str(), |cap: &Captures| {
        match &cap[0] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("We should never get here"),
        }
        .to_string()
    });

    let binding = contents.chars().rev().collect::<String>();
    let result_back = re_back.replace_all(&binding, |cap: &Captures| {
        match &cap[0] {
            "eno" => "1",
            "owt" => "2",
            "eerht" => "3",
            "ruof" => "4",
            "evif" => "5",
            "xis" => "6",
            "neves" => "7",
            "thgie" => "8",
            "enin" => "9",
            _ => panic!("We should never get here"),
        }
        .to_string()
    });
    // println!("{}", result);
    println!("Contents {}", result_front);
    println!("Contents {}", result_back);
    let mut result: HashMap<usize, u32> = HashMap::with_capacity(1000);

    for (front_index, front_line) in result_front.lines().enumerate() {
        println!("{}, {}", front_line, front_index);
        for front_char in front_line.chars() {
            if front_char.is_numeric() {
                result.insert(front_index, front_char.to_digit(10).unwrap());
                println!(
                    "line front_char      {}     {}",
                    front_char.to_digit(10).unwrap(),
                    front_index
                );
                break;
            }
        }
    }

    let length = result_back.lines().count();

    println!("{:?}     {:?}", result, length);
    let mut answer = 0;
    for (back_index, back_line) in result_back.lines().enumerate() {
        println!("{}, {}  ", back_line, back_index);
        let mut line_answer = 0;
        for back_char in back_line.chars() {
            if back_char.is_numeric() {
                let reverse_back_index = length - 1 - back_index;
                let front_char = *result.get(&reverse_back_index).unwrap();
                println!("line back    {}     {}", front_char, reverse_back_index);
                line_answer += front_char * 10 + back_char.to_digit(10).unwrap();
                println!("line answer{}", line_answer);
                break;
            }
        }
        answer += line_answer;
    }
    println!("line answer{}", answer);
}
