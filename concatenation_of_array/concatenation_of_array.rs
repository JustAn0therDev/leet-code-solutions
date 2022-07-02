fn concatenation_of_array(nums: Vec<i32>) -> Vec<i32> {
	let mut ans: Vec<i32> = Vec::<i32>::new();

	let mut idx: usize = 0;

	while idx < nums.len() * 2 {
		ans.push(nums[idx % nums.len()]);
		idx += 1;
	}

	return ans;
}

fn main() {
	let mut input = Vec::<i32>::new();
	input.push(1);
	input.push(2);
	input.push(1);

	println!("{:?}", concatenation_of_array(input));
}