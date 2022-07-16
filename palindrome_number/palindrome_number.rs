fn is_palindrome(x: i32) -> bool {
	x.to_string() == x.to_string().chars().rev().collect::<String>()
}

fn main() {
	println!("{}", is_palindrome(123));
}