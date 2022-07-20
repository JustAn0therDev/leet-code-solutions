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
  	fn append(&mut self, val: i32) {
	    match &mut self.next {
	        None => {
	        	let n = ListNode {
	                    val: val,
	                    next: None,
	                };
	                self.next = Some(Box::new(n));
	        },
	        Some(ref mut x) => x.append(val),
	    }
	}
}

fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut merged_list: Option<Box<ListNode>> = Some(Box::<ListNode>::new(ListNode::new(0)));
    let mut ans: Vec<i32> = Vec::<i32>::new();

    let mut mutable_list1 = list1;
    let mut mutable_list2 = list2;

    loop {
        match mutable_list1 {
            Some(ref node) => { 
            	ans.push(node.val);
            },
            None => { 
            }
        }

        match mutable_list2 {
            Some(ref node) => { 
            	ans.push(node.val);
            },
            None => { 
            }
        }

        match mutable_list1 {
        	Some(node) => mutable_list1 = node.next,
        	None => {}
        }    

        match mutable_list2 {
        	Some(node) => mutable_list2 = node.next,
        	None => {}
        }

        if mutable_list1 == None && mutable_list2 == None {
        	break;
        }
    }

    ans.sort();

    for i in ans {
    	match merged_list {
    		Some(ref mut node) => node.append(i),
    		None => {}
    	}
    }

    merged_list.unwrap().next
}


fn main() {
	let mut input_one = Some(Box::<ListNode>::new(ListNode::new(2)));

	match input_one {
		Some(ref mut node) => node.next = Some(Box::<ListNode>::new(ListNode::new(3))),
		None => {}
	}

	let mut input_two = Some(Box::<ListNode>::new(ListNode::new(5)));

	match input_two {
		Some(ref mut node) => node.next = Some(Box::<ListNode>::new(ListNode::new(1))),
		None => {}
	}

	println!("{:?}", merge_two_lists(input_one, input_two));
}