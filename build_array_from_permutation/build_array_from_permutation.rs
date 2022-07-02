fn build_array_from_permutation(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::<i32>::new();

    let mut idx: usize = 0;

    while idx < nums.len() {
        ans.push(nums[nums[idx] as usize]);
        idx += 1;
    }

    return ans;
}

fn main() {
	// println!("{:?}", build_array_from_permutation());
}