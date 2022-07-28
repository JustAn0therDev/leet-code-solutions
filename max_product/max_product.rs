fn max_product(nums: Vec<i32>) -> Vec<i32> {
	let mut ans: i32 = -1;

	let mut i = 0;

	while i < nums.len() {
		let mut j = 0;
		while j < nums.len() {
			let current = (nums[i] - 1) * (nums[j] - 1);
			if i != j && current > ans {
				ans = current;
			}

			j += 1;
		}

		i += 1;
	}

	ans
}

fn main() {

}