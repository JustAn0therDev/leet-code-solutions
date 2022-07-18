use std::collections::HashMap;

fn single_number(nums: Vec<i32>) -> i32 {
	let mut ans = HashMap::<i32, i32>::new();

	for num in nums {
		if ans.contains_key(&num) {
			*ans.get_mut(&num).unwrap() += 1;
		} else {
			ans.insert(num, 1);
		}
	}

	for k in ans.keys() {
		if ans[k] == 1 {
			return *k;
		}
	}

	0
}

fn main() {
	let input = vec![1,1,2,2,3,4,5,6,2,3,4,5,2];
	println!("{:?}", single_number(input)); // should be 1
}