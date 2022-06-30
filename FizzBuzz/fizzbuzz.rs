fn fizz_buzz(n: i32) -> Vec<String> {
	let mut ans: Vec<String> = Vec::<String>::new();

	let mut i = 1;

	while i <= n {
		if i % 15 == 0 {
			ans.push("FizzBuzz".to_string());
		} else if i % 5 == 0 {
			ans.push("Buzz".to_string());
		} else if i % 3 == 0 {
			ans.push("Fizz".to_string());
		} else {
			ans.push(i.to_string());
		}

		i += 1;
	}

	return ans;
}

fn main() {
	println!("{:?}", fizz_buzz(15));
}

