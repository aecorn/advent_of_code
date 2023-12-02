use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    part1();
    part2();
}


fn part2() {
    
    let mut sum: i32 = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut first: char = 'a';
        let mut last: char = 'a';
        print!("{}\n", line);
        let line = line.replace("one", "o1e")
            .replace("two", "t2e")
            .replace("three", "th3ee")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9ne");


        for c in line.chars() {
            if c.is_digit(10) {
                last = c;
                if first == 'a' {
                    first = c;
                }
            }
        }
        let line_sum: String = [first, last].iter().collect();
        sum = sum + line_sum.parse::<i32>().unwrap();
        print!("{}\n", line);
        print!("{}\n", line_sum);
    }
    println!("{}", sum);
}


fn part1() {
    let file = File::open("part1_input.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut sum: i32 = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut first: char = 'a';
        let mut last: char = 'a';
        for c in line.chars() {
            if c.is_digit(10) {
                last = c;
                if first == 'a' {
                    first = c;
                }
            }
        }
        let line_sum: String = [first, last].iter().collect();
        sum = sum + line_sum.parse::<i32>().unwrap();
        //print!("{}\n", line_sum);
    }
    println!("{}", sum);
}