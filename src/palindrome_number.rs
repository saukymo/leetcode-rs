pub struct Solution;

const P10: &[i32] = &[1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { 
            return false;
        }
        if x == 0 {
            return true;
        }

        let mut divide = P10[(x as f64).log10().floor() as usize];

        let mut n = x;
        while n > 0 {
            println!("{:}, {:}", n, divide);

            let h = n / divide;
            let e = n % 10;

            println!("{:}, {:}", h, e);

            if h != e {
                return false
            }

            n = n % divide / 10;
            divide = divide / 100;
        }

        return true;
    }
}