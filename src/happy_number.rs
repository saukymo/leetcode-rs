pub struct Solution {

}

use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut p = n;
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(p);

        loop {
            let mut sum = 0;
            while p > 0 {
                let z = p.rem_euclid(10);
                sum += z * z;
                p = p / 10;
            }

            if sum == 1 {
                return true;
            }

            if set.contains(&sum) {
                break;
            }

            set.insert(sum);
            
            p = sum;
        }

        return false;
    }
}