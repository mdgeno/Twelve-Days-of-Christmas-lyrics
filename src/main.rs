use std::collections::HashMap;

fn main(){

	let mut lyrics = HashMap::new();	
	lyrics.insert(1, "a partridge in a pear tree");
	lyrics.insert(2, "two turtle doves and");
	lyrics.insert(3, "three french hens");
	lyrics.insert(4, "four calling birds");
	lyrics.insert(5, "five golden rings");
	lyrics.insert(6, "six geese a-laying");
	lyrics.insert(7, "seven swans a-swimming");
	lyrics.insert(8, "eight maids a-milking");
	lyrics.insert(9, "nine ladies dancing");
	lyrics.insert(10, "ten lords a-leaping");
	lyrics.insert(11, "eleven pipers piping");
	lyrics.insert(12, "twelve drummers drumming");

	let mut day = 1;
	while day<=12{
		println!("On the {} day of Christmas, my true love sent to me, ", nth(&day));
		print_gifts(&lyrics, day);
		day+=1;
	}
	
}

fn nth(d: &i32) -> String{
	match d{
		1 => d.to_string()+"st",
		2 => d.to_string()+"nd",
		3 => d.to_string()+"rd",
		_ => d.to_string()+"th"
	}
}

fn print_gifts(gifts: &HashMap<i32, &str>, mut day: i32){	
	while day>0{
		match gifts.get(&day){
			Some(val) => println!("{val}"),
			None => println!("no gift")
		}
		day-=1;
	}
	println!(" ");	
}
