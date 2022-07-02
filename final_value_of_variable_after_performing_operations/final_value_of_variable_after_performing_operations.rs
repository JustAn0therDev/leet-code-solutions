fn final_value_of_variable_after_performing_operations(operations: Vec<String>) -> i32 {
	let mut ans: i32 = 0;

	for i in operations {
		if i.contains("+") {
			ans += 1;
		} else {
			ans -= 1;
		}
	}

	return ans;
}

fn main() {
	let mut input = Vec::<String>::new();
	input.push("X++".to_string());
	input.push("X++".to_string());
	input.push("X++".to_string());
	input.push("X--".to_string());

	println!("{:?}", final_value_of_variable_after_performing_operations(input));
}