/*  Reverse String 
    https://exercism.org/tracks/rust/exercises/reverse-string
*/
pub fn reverse(input: &str) -> String {
   // todo!("Write a function to reverse {input}");
    let  word = input.trim();
	let mut reverse_input: String = String::new();
	if input.is_empty() {
		println!("No input");	
	}else if word.len() == 1{
		println!("Only one char");
		reverse_input = word.to_string();	
/*
  }else if input.is_ascii() {
		println!("is_ascii");
		while i > 0 {
			reverse_input.push_str(&word[i-1..i]);
			i-=1;
		}	
*/
	}else{
		let x: String =  word.chars().rev().collect();
		reverse_input.push_str(&x);
	}
	println!("{reverse_input}");
	reverse_input.to_string()
}

fn main(){
	reverse("自来也 , うずまき ナルト");
}
