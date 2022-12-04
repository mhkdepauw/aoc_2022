use aoc_rust::{read_lines, regex_parse_as_i32};

fn main() {
    let lines = read_lines(2022,4);
    //part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let mut counter = 0;
    for line in lines{
        let nums = regex_parse_as_i32(&line, r"(\d+)-(\d+),(\d+)-(\d+)");
        if nums[0] >= nums[2] && nums[1] <= nums[3] {
            counter += 1;
        }
        else if nums[0]<= nums[2] && nums[1] >= nums[3] {
            counter += 1;
        }
    }
    println!("{}",counter);
}

fn part2(lines: Vec<String>) {
    let mut counter = 0;
    let mut score_raised:bool;
    for line in lines{
        score_raised = false;
        let nums = regex_parse_as_i32(&line, r"(\d+)-(\d+),(\d+)-(\d+)");
        for &number in nums[..2].iter(){
            if number == nums[2] || number == nums[3] {
                counter += 1;
                score_raised = true;
                break
            }
            else if number > nums[2] && number < nums[3] {
                counter += 1;
                score_raised = true;
                break
            }
        }
        if score_raised {
            continue;
        }
        for &number in nums[2..].iter() {
            if number > nums[0] && number < nums[1] {
                counter += 1;
                break
            }
        }
    }
    println!("{}",counter);
}