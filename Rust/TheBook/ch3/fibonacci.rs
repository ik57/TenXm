/* Generate the nth Fibonacci number.
*/

fn get_nth_fib(nth : i32) -> i32{
		if nth == 0{
			return 0;
		}else if nth == 1{
			return 1;
		}
		let mut fib : i32 = 0;
		let mut f0 : i32 = 0;
		let mut f1 : i32 = 1;
		for _ in 1..nth{
			fib = f0 + f1;
			f0 = f1;
			f1 = fib;
		}
		return fib;
}
fn main(){
	let n = get_nth_fib(7);
	println!("fib = {n}");
}
