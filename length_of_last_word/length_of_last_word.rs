fn length_of_last_word(s: String) -> i32 {
	s.split(" ").filter(|&s| s != "").collect::<Vec<&str>>().last().unwrap().len() as i32
}

fn main() {
	println!("{:?}", length_of_last_word("fly   me to the moon  ".to_string()));
}