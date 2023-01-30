fn climbing_stairs(n: i32) -> i32 {
	if n == 1 || n == 0 {
		return n;
	}
	return climbing_stairs(n - 1) + climbing_stairs(n - 2);
}

fn main() {
	print!("{}", climbing_stairs(3));
}