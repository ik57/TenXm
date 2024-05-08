/* Print the lyrics to the Christmas carol
	 “The Twelve Days of Christmas,” 
	 taking advantage of the repetition in the song.
*/

fn get_gifts(n: usize){
	let gifts = ["a Partridge in a Pear Tree", "2 Turtle Doves, and", "3 French Hens",
	 "4 Calling Birds", "5 Golden Rings", "6 Geese a Laying",
	 "7 Swans a Swimming", "8 Maids a Milking", "9 Ladies Dancing",
	  "10 Lords a Leaping", "11 Pipers Piping", "12 Drummers Drumming"];

	  for i in (0..n+1).rev(){
		println!("{}",gifts[i]);
	  }
}
fn main(){
	let days = ["first","second","third","fourth","fifth",
				"sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
	for n in 0..days.len() {
		println!("\nOn the {} day of Christmas,\nmy true love sent to me:",days[n]);
		get_gifts(n);
	}
}
