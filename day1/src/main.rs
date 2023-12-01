use std::fs;

use regex::{Captures, Regex};

fn main() {
    let file_path = "input.txt";
    println!("Infile {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // let re = Regex::new(r"\bone\b").unwrap();
    // let result = re.replace_all(&contents, "1");

    // let mut result = contents.as_str().replace("one", "1");
    // result = result.as_str().replace("two", "2");
    // result = result.as_str().replace("three", "3");
    // result = result.as_str().replace("four", "4");
    // result = result.as_str().replace("five", "5");
    // result = result.as_str().replace("six", "6");
    // result = result.as_str().replace("seven", "7");
    // result = result.as_str().replace("eight", "8");
    // result = result.as_str().replace("nine", "9");

    let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let result = re.replace_all(contents.as_str(), |cap: &Captures| {
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
    // println!("{}", result);
    println!("Contents {}", result);
    let mut values: Vec<u32> = vec![];
    let mut temp = 0;
    let mut first = true;
    let mut index = 0;
    // let mut char_check = vec![];

    for c in result.chars() {
        if c.is_numeric() {
            let value = c.to_digit(10).unwrap();
            if first {
                values.push(value * 10);
                first = false;
            }
            temp = value;
        }
        //else {
        //     char_check.push(c.to_string());
        //     let is_match = char_check.join("");
        //     let matched_num = match is_match.as_str() {
        //         "one" => 1,
        //         "two" => 2,
        //         "three" => 3,
        //         "four" => 4,
        //         "five" => 5,
        //         "six" => 6,
        //         "seven" => 7,
        //         "eight" => 8,
        //         "nine" => 9,
        //         _ => 0,
        //     };
        //     if matched_num != 0 {
        //         char_check = vec![]
        //     }

        // println!("check:{}, {}", is_match, matched_num);

        if c == '\n' {
            let mut first_letter = values[index];
            first_letter += temp;
            values[index] = first_letter;
            println!("values: {:?}, index:{}", values, index);
            temp = 0;
            index += 1;
            first = true;
        };
    }

    let mut first_letter = values[index];
    first_letter += temp;
    values[index] = first_letter;
    println!("values: {:?}, index:{}", values, index);

    let mut sum = 0;
    for value in values {
        sum += value;
    }

    println!("final answer: {}", sum);
}
