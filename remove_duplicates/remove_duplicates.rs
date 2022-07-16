fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
	let mut ans = Vec::<i32>::new();

	let mut idx = 0;

	while idx < nums.len() {
		if !ans.contains(&nums[idx]) {
			ans.push(nums[idx]);
		}

		idx += 1;
	}

	*nums = ans;

	nums.len() as i32
}

fn main() {
	let mut vector = vec![0,0,1,1,1,2,2,3,3,4];
	println!("{:?}", remove_duplicates(&mut vector));
}