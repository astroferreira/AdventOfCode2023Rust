use std::fs::read_to_string;

fn main() {

    let mut sum: u32 = 0;
    for line in read_to_string("./input.txt").unwrap().lines() {
        let number = extract_numbers(line.to_string());
        sum += number
    }
    println!("{sum}")
}


fn extract_numbers(line: String) -> u32 {

    let english_numbers: [String; 9] = ["one".to_string(),
                                        "two".to_string(), 
                                        "three".to_string(),
                                        "four".to_string(),
                                        "five".to_string(),
                                        "six".to_string(),
                                        "seven".to_string(),
                                        "eight".to_string(),
                                        "nine".to_string()];

    let mut replaceble = line.clone();
    for (i, en) in english_numbers.iter().enumerate() {
        let n =  (i+1).to_string();
        let replacement = en.to_owned() + &n + en;
        replaceble = replaceble.replace(en, &replacement);
    }
    
    let result: String = replaceble.chars()
                                   .filter(|c| c.is_numeric())
                                   .collect();         

    let last = result.chars().last().unwrap();
    let first = result.chars().next().unwrap();

    let number = format!("{}{}", first, last).parse::<u32>().unwrap();

    number
}