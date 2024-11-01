///
///     Day 1, Part One
///     [puzzle link](https://adventofcode.com/2023/day/1#part1)
///     Find the calibration value in the input.txt file
///     by combining the first digit and the last digit
///     (in that order) to form a single two-digit number.
///
///     Consider your entire calibration document(/src/input.txt).
///     What is the sum of all of the calibration values?
///
fn find_calibration_value(line: &str) -> u32{
    let mut first_digit : char = ' ';
    let mut last_digit : char = ' ';
    let line_as_char = line.chars();
    for x in line_as_char{
        if x.is_numeric(){
            if first_digit == ' '{
                first_digit = x;
            }else {
                last_digit = x;
            }
        }
    }
    if last_digit == ' '{
        last_digit = first_digit;
    }
    let mut value: String = first_digit.to_string();
    value.push_str(&last_digit.to_string());
    return value.parse().unwrap() 
}
fn main() {
    let mut sum : u32 = 0;
    for input in include_str!("input.txt").lines() {
        for line in input.lines(){
            sum+= find_calibration_value(line);
        }
    }
    println!("The puzzle answer is: {sum}");
}
