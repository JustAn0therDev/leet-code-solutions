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


fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut ans: Option<Box<ListNode>> = None;

  loop {
    let should_break = match head {
      Some(ref val) => {
        match ans {
          Some(ans_val) => {

          },
          None => {}
        }

        true
      },
      None => false
    };

    if should_break {
      break
    }
  }

  return ans;
}

fn main() {
	// print!("{}", )
}