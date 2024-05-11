/* In Fizz Buzz we want to run through a series of numbers
 (say 1 to 100 inclusive). For each number:

    if the number is divisible by 3, print the word Fizz
    if the number is divisible by 5, print the word Buzz
    if the number is divisible by both 3 and 5, print FizzBuzz
    otherwise, just print the number

    As a starting point, you could use a range to generate the numbers,
    then use a for ... in ... loop to get each number one at a time,
     then some if/else statements to get the output.
  https://rust-simply.rs/idiomatic-rust-in-simple-steps/language-basics/control-flow.html
 */
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let mut array = [1; 12];
    for i in array{
        array[i] = rng.gen_range(1..=100);
        if (array[i] % 3) == 0 && (array[i] % 5) == 0{
            println!("Random number: {} is FizzBuzz", array[i]);
        }else if (array[i] % 3) == 0{
            println!("Random number: {} is divisible by 3", array[i]);
        }else if (array[i] % 5) == 0{
            println!("Random number: {} is divisible by 5", array[i]);
        }else{
            println!("Random number: {}", array[i]);
        }
    }
}
