use std::cmp::min;
use aoc_rust::read_lines_split_on_empty_line_as_i32;

fn main() {
    let megavec = read_lines_split_on_empty_line_as_i32(2022,1);
    part1(megavec.clone());
    part2(megavec.clone());
}

fn part1(vecvec: Vec<Vec<i32>>){
    let mut biggest_number = 0;
    for vec in vecvec{
        let calory_sum = vec.iter().sum();
        if calory_sum > biggest_number{
            biggest_number = calory_sum;
        }
    }
    println!("{}",biggest_number)
}

fn part2(vecvec: Vec<Vec<i32>>){
    let mut stacked_elves = vec![0,0,0];
    for vec in vecvec{
        let calory_sum: i32 = vec.iter().sum();
        let minval = *stacked_elves.iter().min().unwrap();
        if calory_sum > minval{
            stacked_elves.remove(stacked_elves.iter().position(|&el| el == minval).unwrap());
            stacked_elves.push(calory_sum);
            if stacked_elves.len() > 3 {panic!("stacked_elves Vector is wrong size") }
        }
    }
    println!("{}",stacked_elves.iter().sum::<i32>())
}