fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
	let mut ans: Vec<bool> = Vec::new();

	let mut greatest_amount_of_candies = -1;

	let mut idx = 0;

	while idx < candies.len() {
		if candies[idx] > greatest_amount_of_candies {
			greatest_amount_of_candies = candies[idx];
		}

		idx += 1;
	}

	idx = 0;

	while idx < candies.len() {
		if (candies[idx] + extra_candies) >= greatest_amount_of_candies {
			ans.push(true);
		} else {
			ans.push(false);	
		}
		
		idx += 1;
	}

	ans
}

fn main() {
	println!("{:?}", kids_with_candies(vec![2,3,5,1,3], 3));
}