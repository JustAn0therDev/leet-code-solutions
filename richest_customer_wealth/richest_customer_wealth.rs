fn richest_customer_wealth(accounts: Vec<Vec<i32>>) -> i32 {
	let mut ans = -1;

	for i in accounts {
		let mut current = 0;
		for j in i {
			current += j;
		}

		if current > ans {
			ans = current;
		}
	}

	return ans;
}

fn main() {
	let mut input: Vec<Vec<i32>> = Vec::new();

	let mut first_vec = Vec::<i32>::new();
	first_vec.push(1);
	first_vec.push(5);

	let mut second_vec = Vec::<i32>::new();
	second_vec.push(7);
	second_vec.push(3);

	let mut third_vec = Vec::<i32>::new();
	third_vec.push(3);
	third_vec.push(5);

	input.push(first_vec);
	input.push(second_vec);
	input.push(third_vec);

	println!("{:?}", richest_customer_wealth(input));
}