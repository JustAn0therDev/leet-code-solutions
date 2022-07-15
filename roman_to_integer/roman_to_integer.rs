use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
	let mut ans = 0;

	let mut roman_chars_to_int = HashMap::<&str, i32>::new();
	roman_chars_to_int.insert("I", 1);
	roman_chars_to_int.insert("V", 5);
	roman_chars_to_int.insert("X", 10);
	roman_chars_to_int.insert("L", 50);
	roman_chars_to_int.insert("C", 100);
	roman_chars_to_int.insert("D", 500);
	roman_chars_to_int.insert("M", 1000);
	roman_chars_to_int.insert("IV", 4);
	roman_chars_to_int.insert("IX", 9);
	roman_chars_to_int.insert("XL", 40);
	roman_chars_to_int.insert("XC", 90);
	roman_chars_to_int.insert("CD", 400);
	roman_chars_to_int.insert("CM", 900);

	let char_collection: Vec::<char> = s.chars().collect();

	let mut idx = 0;

	while idx < char_collection.len() {
		if idx < char_collection.len() - 1 {
			let mut current = String::new();

			current.push_str(&char_collection[idx].to_string().as_str());
			current.push_str(&char_collection[idx + 1].to_string().as_str());
		
			if roman_chars_to_int.contains_key(&current.as_str()) {
				ans += roman_chars_to_int[&current.as_str()];
				idx += 2;
				continue;
			}
		}

		if roman_chars_to_int.contains_key(char_collection[idx].to_string().as_str()) {
			ans += roman_chars_to_int[&char_collection[idx].to_string().as_str()];
		}

		idx += 1;
	}

	return ans;

}

fn main() {
	println!("{}", roman_to_int("XVI".to_string()));
}