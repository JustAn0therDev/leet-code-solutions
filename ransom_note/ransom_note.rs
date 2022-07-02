/*
	Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

	Each letter in magazine can only be used once in ransomNote.
*/

use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
	let mut ransom_note_chars = HashMap::<char, i32>::new();
	let mut magazine_chars = HashMap::<char, i32>::new();

	for ch in ransom_note.chars() {
		if ransom_note_chars.contains_key(&ch) {
			*ransom_note_chars.get_mut(&ch).unwrap() += 1;
		} else {
			ransom_note_chars.insert(ch, 1);
		}
	}

	for ch in magazine.chars() {
		if magazine_chars.contains_key(&ch) {
			*magazine_chars.get_mut(&ch).unwrap() += 1;
		} else {
			magazine_chars.insert(ch, 1);
		}
	}

	let mut ans = true;

	for k in ransom_note_chars.keys() {
		if !magazine_chars.contains_key(k) || magazine_chars[k] < ransom_note_chars[k] {
			ans = false;
			break;
		}
	}

	return ans;
}

fn main() {
	println!("{:?}", can_construct("adsadsaa".to_string(), "aab".to_string()));
}