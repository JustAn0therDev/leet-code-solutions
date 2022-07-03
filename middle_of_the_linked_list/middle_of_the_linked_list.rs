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

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut size: i32 = 0;

    let mut reference = &head;

    loop {
        match reference {
            Some(val) => reference = &val.next,
            None => break,
        }

        size += 1;
    }

    let middle_idx: i32 = size / 2;

    let mut idx = 0;

    let mut node: Option<Box<ListNode>> = head;

    while idx < middle_idx {
        match node {
            Some(val) => node = val.next,
            None => break,
        }

        idx += 1;
    }

    return node;
}

fn main() {

}