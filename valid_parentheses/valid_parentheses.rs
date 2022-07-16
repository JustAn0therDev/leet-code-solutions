fn is_valid(s: String) -> bool {
	let mut last_char: char = '\0';

	for ch in s.chars() {
		if ch == '{' || ch == '(' || ch == '[' {
			last_char = ch;
			continue;
		}

		if (ch == '}' && last_char == '{') || (ch == ']' && last_char == '[') || (ch == ')' && last_char == '(') {
			last_char = '\0';
			continue;
		}

		return false;
	}

	last_char == '\0'
}

fn main() {
	println!("{}", is_valid("(){}[]".to_string())); // should be true
	println!("{}", is_valid("()[}()".to_string())); // should be false
}