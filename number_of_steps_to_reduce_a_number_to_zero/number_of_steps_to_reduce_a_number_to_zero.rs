/*
	Given an integer num, return the number of steps to reduce it to zero.

	In one step, if the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.
*/

fn number_of_steps_to_reduce_a_number_to_zero(mut n: i32) -> i32 {
	let mut count = 0;

	while n > 0 {
		if n % 2 == 0 {
			n = (n / 2) as i32;
		} else {
			n -= 1;
		}

		count += 1;
	}

	return count;
}

fn main() {
	println!("Should be 6: {:?}", number_of_steps_to_reduce_a_number_to_zero(14));
}