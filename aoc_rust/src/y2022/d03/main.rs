use aoc_rust::read_lines;

fn main() {
    let lines = read_lines(2022,3);
    part1(lines.clone());
    part2(lines.clone());
}

fn part1(item_lines: Vec<String>) {
    let mut score = 0;
    for line in item_lines{
        let linevec = line.split_at(line.len()/2);
        let mut found_doubles:Vec<char> = Vec::new();
        for letter in linevec.0.chars(){
            if linevec.1.contains(letter) && !found_doubles.contains(&letter){
                //println!("{} is in {} and in {}",letter, linevec.0, linevec.1);
                found_doubles.push(letter);
                if letter.is_uppercase() {
                    score += 26;
                }
                score += (letter.to_ascii_lowercase() as i32)-('a' as i32-1);
            }
        }
    }
    println!("{}", score);
}

fn part2(item_lines: Vec<String>) {
    let mut score = 0;
    for i in 1..item_lines.len(){
        if ((i+1) % 3) == 0{
            let mut found_doubles:Vec<char> = Vec::new();
            for letter in item_lines[i].chars(){
                if item_lines[i-1].contains(letter) && item_lines[i-2].contains(letter) && !found_doubles.contains(&letter) {
                    found_doubles.push(letter);
                    if letter.is_uppercase() {
                        score += 26;
                    }
                    score += (letter.to_ascii_lowercase() as i32)-('a' as i32-1);
                }
            }
        }
    }
    println!("{}",score);
}