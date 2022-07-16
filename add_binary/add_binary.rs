fn add_binary(a: String, b: String) -> String {
    let ans = u128::from_str_radix(&a.as_str(), 2).unwrap() + u128::from_str_radix(&b.as_str(), 2).unwrap();

    format!("{ans:b}")
}

fn main() {
	println!("{}", add_binary("11", "1"));
}