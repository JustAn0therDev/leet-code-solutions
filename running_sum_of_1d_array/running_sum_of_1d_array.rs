fn running_sum(nums: Vec<i32>) -> Vec<i32> {
	let mut ans = Vec::<i32>::new();

	let mut idx: usize = 0;

	while idx < nums.len() {
		let mut inner_idx: i32 = idx as i32;
		let mut sum: i32 = 0;

		while inner_idx >= 0 {
			sum += nums[inner_idx as usize];
			inner_idx -= 1;
		}

		ans.push(sum);
		idx += 1;
	}

	return ans;
}

fn main() {
	// println!("{:?}", running_sum());
}