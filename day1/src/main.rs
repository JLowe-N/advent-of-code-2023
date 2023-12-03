use std::fs::read_to_string;

fn main() {
    let is_part_two = true;
    let mut result: i64 = 0;  
    //get_first_and_last_digits("abc1dng2lkga3alg".to_string());

    for line in read_to_string("example2.txt").unwrap().lines() {
        let parsed_line;
        if is_part_two {
            parsed_line = replace_digit_words_with_nums(line.to_string());
        } else {
            parsed_line = line.to_string();
        }
        result += get_first_and_last_digits(parsed_line);
    }
    println!("Result of lines: {result}");
}

fn replace_digit_words_with_nums(search_text: String) -> String {
    // global replace won't necessarily work
    // We should only count the FIRST occurence of a digit or number word
    // Same applies for last
    const NUMBER_WORDS: [(&str, &str); 10] = [("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
    let mut parsed_string: String = search_text.clone();
    for (number_word, digit) in NUMBER_WORDS {
        parsed_string = parsed_string.replace(number_word, digit);
        println!("{parsed_string}");
    }
    parsed_string
}

fn get_first_and_last_digits(search_text: String) -> i64 {
    let mut combination: String = "".to_owned();
    for found in search_text.chars() {
        if found.is_numeric() {
            combination.push_str(&found.to_string());
            break
        }
    }
    for found in search_text.chars().rev() {
        if found.is_numeric() {
            combination.push_str(&found.to_string());
            break
        }
    }
    println!("Combination of first and last digits: {combination}");
    combination.parse().unwrap()
}
