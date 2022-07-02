fn most_words_found(sentences: Vec<String>) -> i32 {
	let mut ans: i32 = -1;

	for i in sentences {
		let current: i32 = (i.matches(" ").count() as i32) + 1;
		if current > ans {
			ans = current;
		}
	}

	return ans;
}

fn main() {
	// println!("{:?}", most_words_found());
}