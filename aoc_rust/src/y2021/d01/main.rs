use aoc_rust::{read_lines_as_i32};

fn main() {
    let lijst :Vec<i32> = read_lines_as_i32(2021, 1);
    let mut counter = 0;
    for element in 1..lijst.len(){
        if lijst[element-1] < lijst[element] {
            counter += 1;
        }
    }
    println!("There are {} increases!",counter);
    counter = 0;
    for element in 3..lijst.len() {
        if lijst[element-3]+lijst[element-2]+lijst[element-1]
            < lijst[element-2]+lijst[element-1]+lijst[element]{
            counter += 1;
        }
    }
    println!("There are {} increases in pairs of 3!",counter);
}