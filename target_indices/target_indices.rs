fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort();

    let mut ans = Vec::<i32>::new();

    let mut idx = 0;

    while idx < nums.len() {

        if nums[idx] == target {
            ans.push(idx as i32);
        }

        idx += 1;
    }


    return ans;
}

fn main() {
    println!("{:?}", target_indices(vec![1,2,3,4,5,2], 2));
}