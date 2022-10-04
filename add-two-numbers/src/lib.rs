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

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        resolve(l1, l2, 0)
    }
}

fn resolve(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, remaining: i32) -> Option<Box<ListNode>> {
    let (v1, n1, p1) = l1
        .map_or((0, None, false), |l| (l.val, l.next, true));
    let (v2, n2, p2) = l2
        .map_or((0, None, false), |l| (l.val, l.next, true));
    let next = v1 + v2 + remaining;
    if next == 0 && n1.is_none() && n2.is_none() {
        if p1 && p2 {
            Some(Box::new(ListNode::new(0)))
        } else {
            None
        }
    } else {
        let next_remaining = next / 10;
        let next_val = next % 10;

        let next_node = resolve(n1, n2, next_remaining);
        Some(Box::new(ListNode {
            val: next_val,
            next: next_node,
        }))
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_gen {
        ($($t:ident: $l1:expr, $l2:expr, $expected:expr, )*) => {
            $(
                #[test]
                fn $t() {
                    let l1 = $l1;
                    let l2 = $l2;
                    let expected = $expected;
                    let result = Solution::add_two_numbers(Some(vec_to_list(&l1)), Some(vec_to_list(&l2)));
                    assert_eq!(result, Some(vec_to_list(&expected)));
                }
            )*
        }
    }

    test_gen! {
        case1: vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8],
        case2: vec![0], vec![0], vec![0],
        case3: vec![9,9,9,9,9,9,9], vec![9,9,9,9], vec![8,9,9,9,0,0,0,1],
    }

    fn vec_to_list(input: &[i32]) -> Box<ListNode> {
        if input.len() == 1 {
            let i = input.first().unwrap().to_owned();
            Box::new(ListNode::new(i))
        } else if input.len() == 2 {
            let last = ListNode::new(input.last().unwrap().to_owned());
            Box::new(ListNode {
                val: input.first().unwrap().to_owned(),
                next: Some(Box::new(last)),
            })
        } else {
            assert_ne!(input.len(), 0);
            let (i, list) = input.split_first().unwrap();
            let next = vec_to_list(list);
            Box::new(ListNode {
                val: i.to_owned(),
                next: Some(next),
            })
        }
    }
}
