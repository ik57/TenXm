/*
	Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/

fn main(){
	let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
	let things = ["a Partridge in a Pear Tree", "2 Turtle Doves, and", "3 French Hens", "4 Calling Birds", "5 Golden Rings", "6 Geese a Laying",
				   "7 Swans a Swimming", "8 Maids a Milking", "9 Ladies Dancing", "10 Lords a Leaping", "11 Pipers Piping", "12 Drummers Drumming"];
	let days_len: usize = days.len();
	for d in 0..days_len {
		println!("On the {} day of Christmas", days[d]);
		println!("my true love sent to me:");

		let mut index: i8 = d as i8;
		while index > -1{
			println!("{}", things[index as usize]);
			index -=1;
		}

		println!("\n");
	}
}