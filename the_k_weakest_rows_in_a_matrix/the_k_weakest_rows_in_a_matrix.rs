/*
	You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing civilians). The soldiers are positioned in front of the civilians. That is, all the 1's will appear to the left of all the 0's in each row.

	A row i is weaker than a row j if one of the following is true:

	The number of soldiers in row i is less than the number of soldiers in row j.
	Both rows have the same number of soldiers and i < j.
	Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.
*/

use std::collections::HashMap;

// NOTE(Ruan): Apparently, this is the fastest solution in Rust for this specific problem in LeetCode.
fn the_k_weakest_rows_in_a_matrix(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
	let mut hash_ans = HashMap::<i32, i32>::new();
	let mut ans: Vec<i32> = Vec::<i32>::new();

	let mut idx = 0;

	while idx < mat.len() {
		let mut current_sum = 0;
		
		for j in &mat[idx] {
			current_sum += j;
		}

		hash_ans.insert(idx as i32, current_sum);

		idx += 1;
	}

	let mut collection_hash: Vec<(&i32, &i32)> = hash_ans.iter().collect();

	collection_hash.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));

	let mut final_collection: Vec<i32> = collection_hash.into_iter().map(|a| a.0).cloned().collect();
	
	idx = 0;

	while idx < k as usize {
		ans.push(final_collection[idx]);
		idx += 1;
	}

	return ans;
}

fn main() {
	// println!("{:?}", the_k_weakest_rows_in_a_matrix());
}