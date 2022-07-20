fn str_str(needle: String, haystack: String) -> i32 {
	let mut left = 0;
	let mut right = needle.len();

	let mut found = false;

	while right <= haystack.len() {
		if needle == &haystack[left..right] {
			found = true;
			break;
		}

		left += 1;
		right += 1;
	}

	if !found {
		return -1;
	}

	return left as i32;
}

fn main() {
	println!("{}", str_str("lo".to_string(), "hello".to_string()));
}