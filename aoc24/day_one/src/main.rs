// Day one, part one.
// https://adventofcode.com/2024/day/1
// 1) Pair up the smallest number in the left list with the smallest number in the right list.
// 2) Find the total distance between the left list and the right list in 'input.txt'. 
// 3) Add up the distances between all of the pairs you found.
// What is the total distance between your lists?

fn main() {
    let mut left_loc: Vec<i32> = Vec::new();
    let mut right_loc: Vec<i32> = Vec::new();
    
    for input in include_str!("input.txt").lines() {
        for line in input.lines(){
            let mut x = line.split_whitespace();
            left_loc.push(x.next().unwrap().parse::<i32>().unwrap(),);
            right_loc.push(x.next().unwrap().parse::<i32>().unwrap(),);
        }
    }
    right_loc.sort();
    left_loc.sort();
    
    let mut answer: i32 = 0;
    for i in 0..left_loc.len(){
         answer += (right_loc[i] - left_loc[i]).abs();
    }
    println!("The puzzle answer: {answer}");
}
