#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1_is_done = false;
    let mut l2_is_done = false;
    let mut result = None;
    let mut result_tmp = &mut result;
    let mut l1 = l1;
    let mut l2 = l2;
    let mut l1_val = 0;
    let mut l2_val = 0;
    let mut ret = 0;

    loop {
        match l1 {
            Some(current) => {
                l1_val = current.val;
                l1 = current.next;
            }
            None => {
                l1_is_done = true;
                l1_val = 0;
            }
        }

        match l2 {
            Some(current) => {
                l2_val = current.val;
                l2 = current.next;
            }
            None => {
                l2_is_done = true;
                l2_val = 0;
            }
        }

        if l1_is_done && l2_is_done {
            if ret > 0 {
                (*result_tmp) = Some(Box::new(ListNode::new(ret)));
            }
            break;
        }

        let tmp = l1_val + l2_val + ret;

        (*result_tmp) = Some(Box::new(ListNode::new(tmp % 10)));
        result_tmp = &mut result_tmp.as_mut().unwrap().next;

        if tmp >= 10 {
            ret = 1
        }
        else {
            ret = 0;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn same_lenght() {
        let l1 = Some(Box::new(super::ListNode {
            val: 2,
            next: Some(Box::new(super::ListNode {
                val: 4,
                next: Some(Box::new(super::ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));
        let l2 = Some(Box::new(super::ListNode {
            val: 5,
            next: Some(Box::new(super::ListNode {
                val: 6,
                next: Some(Box::new(super::ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let result = super::add_two_numbers(l1, l2);

        assert_eq!(result, Some(Box::new(super::ListNode {
            val: 7,
            next: Some(Box::new(super::ListNode {
                val: 0,
                next: Some(Box::new(super::ListNode {
                    val: 8,
                    next: None
                }))
            }))
        })));
    }

    #[test]
    fn l1_longer() {
        let l1 = Some(Box::new(super::ListNode {
            val: 2,
            next: Some(Box::new(super::ListNode {
                val: 4,
                next: Some(Box::new(super::ListNode {
                    val: 3,
                    next: None
                }))
            }))
        }));
        let l2 = Some(Box::new(super::ListNode {
            val: 5,
            next: Some(Box::new(super::ListNode {
                val: 6,
                next: None
            }))
        }));

        let result = super::add_two_numbers(l1, l2);

        assert_eq!(result, Some(Box::new(super::ListNode {
            val: 7,
            next: Some(Box::new(super::ListNode {
                val: 0,
                next: Some(Box::new(super::ListNode {
                    val: 4,
                    next: None
                }))
            }))
        })));
    }

    #[test]
    fn l2_longer() {
        let l1 = Some(Box::new(super::ListNode {
            val: 2,
            next: Some(Box::new(super::ListNode {
                val: 4,
                next: None
            }))
        }));
        let l2 = Some(Box::new(super::ListNode {
            val: 5,
            next: Some(Box::new(super::ListNode {
                val: 6,
                next: Some(Box::new(super::ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }));

        let result = super::add_two_numbers(l1, l2);

        assert_eq!(result, Some(Box::new(super::ListNode {
            val: 7,
            next: Some(Box::new(super::ListNode {
                val: 0,
                next: Some(Box::new(super::ListNode {
                    val: 5,
                    next: None
                }))
            }))
        })));
    }

    #[test]
    fn ending_by_ret() {
        let l1 = Some(Box::new(super::ListNode {
            val: 5,
            next: None
        }));
        let l2 = Some(Box::new(super::ListNode {
            val: 5,
            next: None
        }));

        let result = super::add_two_numbers(l1, l2);

        assert_eq!(result, Some(Box::new(super::ListNode {
            val: 0,
            next: Some(Box::new(super::ListNode {
                val: 1,
                next: None
            }))
        })));
    }
}