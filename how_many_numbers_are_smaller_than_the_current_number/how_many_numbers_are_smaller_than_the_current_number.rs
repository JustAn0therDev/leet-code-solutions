fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
	let mut ans: Vec<i32> = Vec::new();

	let mut idx = 0;

	while idx < nums.len() {
		ans.push(0);
		let mut inner_idx = 0;
		while inner_idx < nums.len() {
			if nums[idx] > nums[inner_idx] {
				ans[idx] += 1;
			}
			inner_idx += 1;
		}
		idx += 1;
	}

	return ans;
}

fn main() {
	println!("{:?}", smaller_numbers_than_current(vec!(8,1,2,2,3)));
}