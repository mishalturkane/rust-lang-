struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() - 1;

        while i >= 0 {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
            if i == 0 {
                break; // Terminate the loop when i reaches 0
            }
            i -= 1;
        }

        digits.insert(0, 1);
        digits
    }
}

fn main() {
    let arr = vec![9];

    println!("{:?}", Solution::plus_one(arr));
}
