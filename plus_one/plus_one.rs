fn plus_one(digits: Vec<i32>) -> Vec<i32> {
	let mut ans = Vec::<i32>::new();

	let mut idx = (digits.len() - 1) as i32;

	let mut carry: i32 = 0;

	while idx > -1 {
		let to_add: i32;
		let mut current = digits[idx as usize];

		if carry == 1 || idx == ((digits.len() - 1) as i32) {
			to_add = 1;
		} else {
			carry = 0;
			to_add = 0;
		}

		current += to_add;

		if current > 9 {
			carry = 1;
			current = 0;
		} else {
			carry = 0;
		}

		ans.push(current);

		if idx == 0 && carry == 1 && current == 0 {
			ans.push(1);
		}

		idx -= 1;
	}

	ans.reverse();

	ans
}

fn main() {
	println!("Should be [1, 2, 4]: {:?}", plus_one(vec![1,2,3]));
	println!("Should be [4, 3, 2, 2]: {:?}", plus_one(vec![4,3,2,1]));
	println!("Should be [1, 0]: {:?}", plus_one(vec![9]));
	println!("Should be [1, 0, 0, 0]: {:?}", plus_one(vec![9,9,9]));
	println!("Should be [9, 0, 0, 0]: {:?}", plus_one(vec![8,9,9,9]));
	println!("Should be [9, 9, 0]: {:?}", plus_one(vec![9, 8, 9]));
}