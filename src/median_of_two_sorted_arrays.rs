pub struct Solution {

}

use std::cmp;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() < nums2.len() {
            Self::find_median_in_smaller_sorted_arrays(&nums1, &nums2)
        } else {
            Self::find_median_in_smaller_sorted_arrays(&nums2, &nums1)
        }
    }

    fn find_median_in_smaller_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
        let n = nums1.len();
        let m = nums2.len();

        let total = n + m;
        let lower_bound = total / 2;

        // range in 0..n
        let mut l = 0;
        let mut r = n; 
        while l < r {
            let m1 = (l + r ) / 2;
            let m2 = lower_bound - m1;

            if m2 > m || (m2 > 0 && m1 < n && nums1[m1] <= nums2[m2 - 1]) {
                l = m1 + 1;
            } else {
                r = m1;
            }

        }

        let m1 = l;
        let m2 = lower_bound - m1;

        if Self::is_odd(total) {
            return Self::find_least(&nums1, &nums2, m1, m2) as f64;
        } else {
            return (Self::find_most(&nums1, &nums2, m1, m2) + Self::find_least(&nums1, &nums2, m1, m2)) as f64 / 2.0;
        }
    }

    fn is_odd(len: usize) -> bool {
        return len&1 != 0;
    }

    fn find_least(nums1: &Vec<i32>, nums2: &Vec<i32>, m1: usize, m2: usize) -> i32 {
        if m1 == nums1.len() {
            return nums2[m2];
        }

        if m2 == nums2.len() {
            return nums1[m1];
        }

        return cmp::min(nums1[m1], nums2[m2]);
    }

    fn find_most(nums1: &Vec<i32>, nums2: &Vec<i32>, m1: usize, m2: usize) -> i32 {
        if m1 == 0 {
            return nums2[m2 - 1];
        }

        if m2 == 0 {
            return nums1[m1 - 1];
        }

        return cmp::max(nums1[m1 - 1], nums2[m2 - 1]);
    }


}