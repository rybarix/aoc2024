pub mod solution {
    use std::{collections::HashMap, error::Error};

    pub fn solve(list1: &Vec<u32>, list2: &Vec<u32>) -> Result<u32, Box<dyn Error>> {
        if list1.len() != list2.len() {
            // we have a problem here
            return Err("lists are different size".into());
        }

        let mut list1_copy = list1.clone();
        let mut list2_copy = list2.clone();
        list1_copy.sort();
        list2_copy.sort();

        let mut distance = 0;

        let mut i = 0;
        while i < list1_copy.len() {
            distance += list1_copy[i].abs_diff(list2_copy[i]);
            i = i + 1;
        }
        return Ok(distance);
    }

    pub fn solve2(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
        let mut freq = HashMap::new();

        for n in list1 {
            freq.insert(n, 0);
        }

        for n in list2 {
            if freq.contains_key(n) {
                let cv = freq.get(n).unwrap();
                freq.insert(n, cv + 1);
            }
        }
        let mut sim_score: u32 = 0;
        for n in list1 {
            sim_score += n * freq.get(n).unwrap();
        }

        return sim_score;
    }
}

#[cfg(test)]
mod tests {
    use super::solution::solve;

    #[test]
    fn internal() {
        let d = solve(&vec![4, 3, 2, 1], &vec![10, 9, 8, 7]);
        let de = 6 + 6 + 6 + 6;
        assert_eq!(d.unwrap(), de)
    }
}
