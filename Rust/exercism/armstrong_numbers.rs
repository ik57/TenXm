pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum : u32 = 0;
    let to_pow : u32 = num.to_string().len() as u32;
    for digit in num.to_string().chars() {
        let x : u32 = digit.to_digit(10).unwrap();
        sum += x.pow(to_pow);
    }
    println!("sum is: {sum}");
    sum == num
}
