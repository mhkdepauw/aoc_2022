use aoc_rust::{read_lines};

fn main() {
    let list = read_lines(2021,2);
    part1(list.clone());
    part2(list.clone());
}

fn part1(lijst: Vec<String>){
    let mut horp = 0;
    let mut depth = 0;
    for line in lijst{
        if line.starts_with("forward"){
            horp += line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
        else if line.starts_with("up") {
            depth -= line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
        else {
            depth += line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", horp*depth)
}

fn part2(lijst: Vec<String>){
    let mut horp = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lijst {
        let x = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        if line.starts_with("forward"){
            horp += x;
            depth += aim*x;
        }
        else if line.starts_with("up") {
            aim -= x;
        }
        else {
            aim += x;
        }
    }
    println!("{}",horp*depth)
}