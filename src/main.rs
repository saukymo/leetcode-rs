mod palindrome_number;

fn main() {
    // println!("{:}", 0_f64.log10());
    let result = palindrome_number::Solution::is_palindrome(-1);
    println!("{:}", result)
}