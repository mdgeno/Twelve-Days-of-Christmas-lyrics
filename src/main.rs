fn main(){
	
	let mut lyrics = Vec::new();
	lyrics.insert(0, "twelve drummers drumming");
	lyrics.insert(1, "eleven pipers piping");
	lyrics.insert(2, "ten lords a-leaping");
	lyrics.insert(3, "nine ladies dancing");
	lyrics.insert(4, "eight maids a-milking");
	lyrics.insert(5, "seven swans a-swimming");
	lyrics.insert(6, "six geese a-laying");
	lyrics.insert(7, "five golden rings");
	lyrics.insert(8, "four calling birds");
	lyrics.insert(9, "three french hens");
	lyrics.insert(10, "two turtle doves and");
	lyrics.insert(11, "a partridge in a pear tree");

	let mut day = 1;
	while day<13{
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

fn print_gifts(gifts: &Vec<&str>, day: i32){
	let d: usize = day.clone().try_into().unwrap();
	let length: usize = gifts.len().try_into().unwrap();
	let mut start: usize = length-d;	
	while start<gifts.len().try_into().unwrap(){
		println!("{}", gifts[start]);
		start+=1;
	}
	println!(" ");	
}
