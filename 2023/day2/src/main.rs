use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    //part1();
    part2();
}


fn part1() {
    let file = File::open("input.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut sum: i32 = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let line_index = line_split[0].replace("Game ", "").parse::<i32>().unwrap();
        let mut line_impossible = false;
        //println!("{}", line_index);
        let pulls = line_split[1].split(";");
        for pull in pulls {
            let pull = pull.trim();
            let colors = pull.split(",");
            for color in colors {
                let color = color.trim();
                let num = color.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                if color.find("red") != None && num > 12{
                    line_impossible = true;
                    println!("Wont add line {line_index}, because of {num} red balls.");
                    break;
                }
                if color.find("green") != None && num > 13{
                    line_impossible = true;
                    println!("Wont add line {line_index}, because of {num} green balls.");
                    break;
                }
                if color.find("blue") != None && num > 14{
                    line_impossible = true;
                    println!("Wont add line {line_index}, because of {num} blue balls.");
                    break;
                }
                //println!("{num}");
            }
            if line_impossible {
                break;
            }
        
        }
    if !line_impossible {
        println!("Want to add line {}", line_index);
        sum = sum + line_index;

    }
        
    }
    println!("{sum}");
}


fn part2() {
    let file = File::open("input.txt").expect("cannot open file");
    let file = BufReader::new(file);
    let mut sum: i32 = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let line_index = line_split[0].replace("Game ", "").parse::<i32>().unwrap();
        let mut red_max: i32 = 0;
        let mut green_max: i32 = 0;
        let mut blue_max: i32 = 0;
        //println!("{}", line_index);
        let pulls = line_split[1].split(";");
        for pull in pulls {
            let pull = pull.trim();
            let colors = pull.split(",");
            for color in colors {
                let color = color.trim();
                let num = color.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                if color.find("red") != None && num > red_max{
                    red_max = num;
                }
                if color.find("green") != None && num > green_max{
                    green_max = num;
                }
                if color.find("blue") != None && num > blue_max{
                    println!("Found blue {color} {num}");
                    blue_max = num;
                }
            }
        
        }
    let powered = red_max * green_max * blue_max;
    
    sum = sum + powered;
    println!("{line_index} - {red_max} {green_max} {blue_max} = {powered} > {sum}");
        
    }
    println!("{sum}");
}