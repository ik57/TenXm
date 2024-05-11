/*  Reverse String 
    https://exercism.org/tracks/rust/exercises/reverse-string
*/
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
   // todo!("Write a function to reverse {input}");
   input.graphemes(true).rev().collect()
}
fn main(){
	reverse("うずまき ナルト");
}
