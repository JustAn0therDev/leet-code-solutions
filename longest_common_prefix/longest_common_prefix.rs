fn longest_common_prefix(strs: Vec<String>) -> String {
	let mut ans: &str = "";
	let mut idx = strs[0].len();

	while idx >= 0 {
		ans = &strs[0][..idx];

		let mut inner_idx = 1;

		let mut valid = true;
		
		while inner_idx < strs.len() {
			if !strs[inner_idx].starts_with(&ans) {
				valid = false;
				break;
			}

			inner_idx += 1;
		}

		if valid {
			break;
		}

		idx -= 1;
	}

	return (*ans).to_string();
}

fn main() {
	println!("{}", longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
}