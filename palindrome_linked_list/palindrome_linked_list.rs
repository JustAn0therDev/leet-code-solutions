#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut ans = String::new();

    let mut mutable_head = head;

    while mutable_head != None {
        match mutable_head {
            Some(ref value) => ans.push_str(&value.val.to_string()),
            None => break
        }

        mutable_head = mutable_head.unwrap().next;
    }

    let reversed_ans = ans.chars().rev().collect::<String>();

    return ans == reversed_ans;
}


fn main() {
	// is_palindrome call
}