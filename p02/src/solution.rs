pub mod solution {
    pub fn is_safe(list: &Vec<u32>) -> bool {
        let mut is_increasing = false;

        if list.len() < 2 {
            return false;
        }

        if list[0] < list[1] {
            // decreasing
            is_increasing = false;
        }

        if list[0] > list[1] {
            // increasing
            is_increasing = true;
        }

        if list[0] == list[1] {
            return false;
        }

        for i in 0..(list.len() - 1) {
            let a1 = list[i];
            let a2 = list[i + 1];

            // The levels are either all increasing or all decreasing.
            if (is_increasing && a1 < a2) || (!is_increasing && a1 > a2) {
                return false;
            }

            // Any two adjacent levels differ by at least one and at most three.
            if a1.abs_diff(a2) > 3 || a1.abs_diff(a2) == 0 {
                return false;
            }
        }

        return true;
    }

    pub struct LeaveOneOutVec {
        pub items: Vec<u32>,
        pub leave_out: usize,
        pub curr: usize,
    }

    impl Iterator for LeaveOneOutVec {
        type Item = u32;
        // Here, we define the sequence using `.curr` and `.next`.
        // The return type is `Option<T>`:
        //     * When the `Iterator` is finished, `None` is returned.
        //     * Otherwise, the next value is wrapped in `Some` and returned.
        // We use Self::Item in the return type, so we can change
        // the type without having to update the function signatures.
        fn next(&mut self) -> Option<Self::Item> {
            // Leave the index out
            if self.curr == self.leave_out {
                self.curr += 1;
            }

            if self.curr >= self.items.len() {
                return None;
            }

            let current = self.items[self.curr];
            self.curr += 1;

            Some(current)
        }
    }

    /**
     * Iterates over numbers and everytime we try to leave one number out and check safety.
     */
    pub fn is_safe_double_check(list: &Vec<u32>) -> bool {
        let len = list.len();

        for i in 0..len {
            let loov = LeaveOneOutVec {
                items: list.clone(), // can be optimized
                leave_out: i,
                curr: 0,
            };

            let loov_list: Vec<u32> = loov.collect();
            if is_safe(&loov_list) {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::solution::is_safe_double_check;

    use super::solution::{is_safe, LeaveOneOutVec};

    #[test]
    fn test_example_safe() {
        let ex1 = vec![7, 6, 4, 2, 1];
        let ex6 = vec![1, 3, 6, 7, 9];
        let r1 = is_safe(&ex1);
        let r6 = is_safe(&ex6);
        assert_eq!(r1, true, "list ex1 is safe");
        assert_eq!(r6, true, "list ex6 is safe");
    }
    #[test]
    fn test_example_unsafe() {
        let ex2 = vec![1, 2, 7, 8, 9];
        let ex3 = vec![9, 7, 6, 2, 1];
        let ex4 = vec![1, 3, 2, 4, 5];
        let ex5 = vec![8, 6, 4, 4, 1];
        let r2 = is_safe(&ex2);
        let r3 = is_safe(&ex3);
        let r4 = is_safe(&ex4);
        let r5 = is_safe(&ex5);
        assert_eq!(r2, false, "list ex2 is unsafe");
        assert_eq!(r3, false, "list ex3 is unsafe");
        assert_eq!(r4, false, "list ex4 is unsafe");
        assert_eq!(r5, false, "list ex5 is unsafe");
    }

    #[test]
    fn test_leave_out_iterator() {
        let l = LeaveOneOutVec {
            items: vec![0, 1, 2, 3, 5],
            leave_out: 1,
            curr: 0,
        };

        let k: Vec<u32> = l.collect();
        assert_eq!(k[0], 0);
        assert_eq!(k[1], 2);
        assert_eq!(k[2], 3);
        assert_eq!(k[3], 5);
    }

    #[test]
    fn test_problem_dampener() {
        let ex1 = vec![7, 6, 4, 2, 1]; //: Safe without removing any level.
        let ex2 = vec![1, 2, 7, 8, 9]; //: Unsafe regardless of which level is removed.
        let ex3 = vec![9, 7, 6, 2, 1]; //: Unsafe regardless of which level is removed.
        let ex4 = vec![1, 3, 2, 4, 5]; //: Safe by removing the second level, 3.
        let ex5 = vec![8, 6, 4, 4, 1]; //: Safe by removing the third level, 4.
        let ex6 = vec![1, 3, 6, 7, 9]; //: Safe without removing any level.

        let r1 = is_safe_double_check(&ex1);
        let r2 = is_safe_double_check(&ex2);
        let r3 = is_safe_double_check(&ex3);
        let r4 = is_safe_double_check(&ex4);
        let r5 = is_safe_double_check(&ex5);
        let r6 = is_safe_double_check(&ex6);

        assert_eq!(r1, true);
        assert_eq!(r2, false);
        assert_eq!(r3, false);
        assert_eq!(r4, true);
        assert_eq!(r5, true);
        assert_eq!(r6, true);
    }
}
