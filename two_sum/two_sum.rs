/*
	Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

	You may assume that each input would have exactly one solution, and you may not use the same element twice.

	You can return the answer in any order.
*/

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut ans = Vec::<i32>::new();

	let mut idx = 0;

	while idx < nums.len() {
		let mut inner_idx = 0;
		let mut found = false;

		while inner_idx < nums.len() {

			if idx != inner_idx && nums[idx] + nums[inner_idx] == target {
				ans.push(idx as i32);
				ans.push(inner_idx as i32);
				found = true;
				break;
			}

			inner_idx += 1;
		}

		if found {
			break;
		}

		idx += 1;
	}

	return ans;
}

fn main() {
	let mut input = Vec::<i32>::new();

	input.push(2);
	input.push(7);
	input.push(11);
	input.push(15);

	println!("{:?}", two_sum(input, 9));
}