// TODO: this version is not ok. re-make this in O(log n)

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
	if nums.contains(&target) {
		return nums.iter().position(|&a| a == target).unwrap() as i32;
	}

	let mut idx = 1;

	if target < nums[0] {
		return 0;
	} else if target > nums[nums.len() - 1] {
		return nums.len() as i32;
	}

	while idx < nums.len() - 1 {
		if target < nums[idx + 1] && target < nums[idx] && target > nums[idx - 1] {
			break;
		}

		idx += 1;
	}

	idx as i32
}

fn main() {
	println!("Should be 2: {:?}", search_insert(vec![1,3,5,6], 5));
	println!("Should be 1: {:?}", search_insert(vec![1,3,5,6], 2));
	println!("Should be 4: {:?}", search_insert(vec![1,3,5,6], 7));
	println!("Should be 0: {:?}", search_insert(vec![1,3,5,6], 0));
	println!("Should be 1: {:?}", search_insert(vec![1,3], 2));
	println!("Should be 0: {:?}", search_insert(vec![1,3], 1));
	println!("Should be 2: {:?}", search_insert(vec![1,3], 4));
	println!("Should be 2: {:?}", search_insert(vec![1,3,5], 4));
}