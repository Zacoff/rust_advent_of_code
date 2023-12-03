use std::fs;

pub(crate) fn main() {
    let document_path = "/home/zacoff/rust_advent_of_code/day_1/input_test.txt";
    let contents = fs::read_to_string(document_path)
        .expect("Should have been able to read the file");

    let pred = "\n";
    let input:Vec<&str> = contents.split(pred).collect();
    let input_count: i32 = input.len().try_into().unwrap(); 
    let mut index = 0;

    let mut numbers_to_sum: Vec<i32> = (0..input_count).collect();

    for word in input {
        let parts: Vec<&str> = word.split("").collect();
        let mut number = "".to_string();

        for part in parts {
            let is_numeric = part.parse::<i32>();
            match is_numeric {
                Ok(_) => if number.len() < 2 { number.push_str(part)  } else { number.pop(); number.push_str(part) },
                Err(_) => ()
            }
        }

        if number.len() == 1 {
            number = format!("{}{}", number, number);
        }

        numbers_to_sum[index] = number.parse().unwrap();

        index = index + 1;
    }

    let sum: i32 = numbers_to_sum.iter().sum();

    println!("{}", sum)
}