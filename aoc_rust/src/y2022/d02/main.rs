use aoc_rust::read_lines;

fn main() {
    let binding = read_lines(2022,2);
    let lines:Vec<Vec<&str>>  = binding.iter().map(|l| l.split(" ").collect()).collect();
    part1(lines.clone());
    part2(lines.clone());
}

fn part1(input: Vec<Vec<&str>>) {
    let mut total_score = 0;
    for line in input {
        let opp = ["A", "B", "C"].iter().position(| &c | c == line[0] ).unwrap() as i32 + 1;
        let you = ["X", "Y", "Z"].iter().position(| &c | c == line[1] ).unwrap() as i32 + 1;
        let outcome = (you-opp+3) % 3;
        match outcome {
            0 => total_score += 3,
            1 => total_score += 6,
            2 => total_score +=0,
            _ => panic!{"invalid outcome"}
        };
        total_score += you;
    }
    println!("{}",total_score);
}

fn part2(input: Vec<Vec<&str>>) {
    let mut total_score = 0;
    for line in input{
        let opp = ["A", "B", "C"].iter().position(| &c | c == line[0] ).unwrap() as i32 + 1;
        let outcome = ["Y", "Z", "X"].iter().position(| &c | c == line[1] ).unwrap() as i32;
        /*let outcome = match line[1] {
            "X" => 2,
            "Y" => 0,
            "Z" => 1,
            &_ => panic!{"incorrect outcome"}
        };
        */
        let you = if (opp+outcome)>3 { (opp+outcome)%3} else { opp+outcome };
        match outcome {
            0 => total_score += 3,
            1 => total_score += 6,
            2 => total_score +=0,
            _ => panic!{"invalid outcome"}
        };
        total_score += you;
    }
    println!("{}",total_score)
}