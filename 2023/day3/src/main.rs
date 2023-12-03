use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    part1();
    //part2();
}


fn part1() {
    let file = File::open("input.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut sum: i32 = 0;
    let mut line_index: i32 = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        //println!("{}", line_index);
        let mut char_index: i32 = 0;
        let mut in_num_flip = false;
        let mut digits = String::new();
        let mut num_start_index = 0;
        let mut num_stop_index = 0;
        for c in line.chars(){
            if c.is_digit(10) && !in_num_flip{
                // First number found
                digits.push(c);
                in_num_flip = true;
                num_start_index = char_index;
                println!("Found first digit {digits}");
            } else if in_num_flip && c.is_digit(10) && char_index < 139{
                // Still in number
                digits.push(c);
                println!("Found first digit {digits}");
            } else if in_num_flip && (!c.is_digit(10) || char_index == 139) {
                if char_index == 139 && c.is_digit(10){
                    digits.push(c);
                }
                // Number is finished
                let num_stop_index = char_index - 1;
                let current_num: i32 = digits.parse::<i32>().unwrap();
                println!("Came to last digit of {current_num}");
                //println!("{current_num}");
                if special_is_around(num_start_index, num_stop_index, line_index) {
                    println!("{current_num}");
                    sum += current_num;
                    println!{"Found special around {num_start_index}, {line_index} adding {current_num} to {sum}"}
                } else {
                    println!{"Found no around {num_start_index}, {line_index} not adding {current_num} to {sum}"}
                }
                
                digits = "".to_string();
                num_start_index = 0;
                in_num_flip = false;
            }
            


            char_index += 1 ;
        }
        line_index += 1; 
    }
    println!("{sum}")
}

fn special_is_around(num_start_index: i32, num_stop_index: i32, line_index: i32) -> bool{
    //println!("{num_start_index}, {num_stop_index}, {line_index}");
    // look above
    if line_index > 0 {
        // Diagonal left
        if num_start_index > 0 {
            if check_pos(line_index-1, num_start_index-1){
                return true;
            }
        }
        for i in num_start_index..num_stop_index+1{
            if check_pos(line_index-1, i) {
                return true;
            }
        }
        // Diagonal right
        if num_stop_index < 139 { //Change for large input
            if check_pos(line_index-1, num_stop_index+1){
                return true;
            }
        }

    }
    // look on sides
    // left
    if num_start_index > 0 {
        if check_pos(line_index, num_start_index-1){
            return true;
        }
    }
    // right
    if num_stop_index < 139 { //Change for large input
        if check_pos(line_index, num_stop_index+1){
            return true;
        }
    }

    // look at bottom
    if line_index < 139 {  // Change for large input
       // Diagonal left
       if num_start_index > 0 {
        if check_pos(line_index+1, num_start_index-1){
            return true;
        }
    }
    for i in num_start_index..num_stop_index+1{
        if check_pos(line_index+1, i) {
            return true;
        }
    }
    // Diagonal right
    if num_stop_index < 139 { //Change for large input
        if check_pos(line_index+1, num_stop_index+1){
            return true;
        }
    }
    }

    return false;
}

fn check_pos(line_ind: i32, char_ind: i32) -> bool {
    let file = File::open("input.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut line_index_check: i32 = 0;
    let mut c: char = '0';
    //println!("Check pos {char_ind} - {line_ind}");
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut char_index_check: i32 = 0;
        for ch in line.chars(){
            if char_index_check == char_ind{
                c = ch;
                break;
            }
            char_index_check += 1 ;
        }
        if line_index_check == line_ind{
            break;
        }
        line_index_check += 1;
    }
    //println!("Looking at {c}");
    if !c.is_digit(10) && c != '.' {
        //println!("Found '{c}' special char");
        return true;
    } else {
        //print!("{c}");
    }
    return false;
}