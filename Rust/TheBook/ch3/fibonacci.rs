/* Generate the nth Fibonacci number.
		F0 = 0 and F1 = 1.
	    Fn = Fn-1 + Fn-2
*/

fn main(){
	generate_fibonacci_number(29);
}

fn generate_fibonacci_number(n: u32){
	let mut n1: u32 = 0;
	let mut n2: u32 = 0;
	let mut fib: u32;
	let mut counter: u32 = 0;

	while counter < n {
		if counter == 0{
			print!("{}", 0);
			counter += 1;
		}else if counter == 1{
			print!(",{}", 1);
			n2 = 1;
			counter += 1;
		}else{
			fib = n1 + n2;
			print!(",{}", fib);
			n1 = n2;
			n2 = fib;
			counter += 1;
		}
	}
}