fn is_valid(s: String) -> bool {
    let mut stack: Vec::<char> = Vec::<char>::new();

    for ch in s.chars() {
        if ch == '{' || ch == '(' || ch == '[' {
            stack.push(ch);
            continue;
        }

        if stack.len() > 0 {
            if (ch == '}' && stack.last().unwrap() == &'{') || (ch == ']' && stack.last().unwrap() == &'[') || (ch == ')' && stack.last().unwrap() == &'(') {
                stack.pop();
                continue;
            }
        }

        return false;
    }

    stack.len() == 0
}

fn main() {
	println!("{}", is_valid("(){}[]".to_string())); // should be true
	println!("{}", is_valid("()[}()".to_string())); // should be false
}