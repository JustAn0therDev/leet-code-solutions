fn defang_i_paddr(address: String) -> String {
	return str::replace(&address.as_str(), ".", "[.]");
}

fn main() {
	// println!("{:?}", defang_i_paddr());
}